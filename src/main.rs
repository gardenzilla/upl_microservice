use async_stream::stream;
use chrono::{DateTime, Utc};
use futures::pin_mut;
use futures_util::stream::StreamExt;
use gzlib::proto::upl::upl_server::*;
use gzlib::proto::upl::*;
use packman::*;
use std::{collections::HashMap, env, path::PathBuf};
use tokio::sync::{oneshot, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use upl_microservice::prelude::*;
use upl_microservice::upl::{Location, UplMethods};
use upl_microservice::*;

struct UplService {
  // Active UPLs
  upls: Mutex<VecPack<upl::Upl>>,
  // Archived UPLs
  archive: Mutex<VecPack<upl::Upl>>,
}

impl UplService {
  fn init(upls: VecPack<upl::Upl>, archive: VecPack<upl::Upl>) -> Self {
    Self {
      upls: Mutex::new(upls),
      archive: Mutex::new(archive),
    }
  }
  async fn create_new(&self, r: UplNew) -> ServiceResult<UplObj> {
    // Transform best_before object
    let best_before: Option<DateTime<Utc>> = match r.best_before.len() {
      x if x == 0 => None,
      _ => Some(
        DateTime::parse_from_rfc3339(&r.best_before)
          .map_err(|_| ServiceError::bad_request("A megadott lejárati dátum hibás formátumú!"))?
          .with_timezone(&Utc),
      ),
    };

    let new_upl = upl::Upl::new(
      r.upl_id,
      r.product_id,
      r.product_unit,
      r.sku,
      r.piece,
      r.sku_divisible_amount,
      r.sku_divisible,
      r.sku_net_price,
      upl::VAT::from_str(&r.sku_vat).map_err(|e| ServiceError::bad_request(&e))?,
      r.procurement_id,
      r.procurement_net_price_sku,
      upl::Location::Stock(r.stock_id),
      best_before,
      r.is_opened,
      r.created_by,
    )
    .map_err(|e| ServiceError::bad_request(&e))?;

    // Store new UPL
    self.upls.lock().await.insert(new_upl.clone())?;

    // Return it as UplObj
    Ok(new_upl.into())
  }

  async fn get_bulk(&self, r: BulkRequest) -> ServiceResult<Vec<UplObj>> {
    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| r.upl_ids.contains(&upl.unpack().id))
      .map(|upl| upl.unpack().clone().into())
      .collect::<Vec<UplObj>>();

    Ok(res)
  }

  async fn get_by_id(&self, r: ByIdRequest) -> ServiceResult<UplObj> {
    let res = self.upls.lock().await.find_id(&r.upl_id)?.unpack().clone();
    Ok(res.into())
  }

  async fn get_by_id_archive(&self, r: ByIdRequest) -> ServiceResult<UplObj> {
    // Looking for archived object
    let mut res: UplObj = self
      .archive
      .lock()
      .await
      .find_id(&r.upl_id)?
      .unpack()
      .clone()
      .into();
    // Set UplObj to be archived
    res.is_archived = true;
    // Return UplObj
    Ok(res)
  }

  async fn get_by_sku(&self, r: BySkuRequest) -> ServiceResult<Vec<String>> {
    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| upl.unpack().get_sku() == r.sku)
      .map(|upl| upl.unpack().id.clone())
      .collect::<Vec<String>>();

    Ok(res)
  }

  async fn get_by_sku_and_location(
    &self,
    r: BySkuAndLocationRequest,
  ) -> ServiceResult<Vec<String>> {
    // Determine location
    let location: Location = match r.clone().location.ok_or(ServiceError::internal_error(
      "Nem sikerült a UPL lokációt dekódolni",
    ))? {
      by_sku_and_location_request::Location::Stock(lid) => Location::Stock(lid),
      by_sku_and_location_request::Location::Cart(lid) => Location::Cart(lid),
      by_sku_and_location_request::Location::Delivery(lid) => Location::Delivery(lid),
      by_sku_and_location_request::Location::Discard(lid) => Location::Discard(lid),
    };

    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| {
        let _upl = upl.unpack();
        _upl.get_sku() == r.sku && _upl.location == location
      })
      .map(|upl| upl.unpack().id.clone())
      .collect::<Vec<String>>();

    Ok(res)
  }

  async fn get_by_location(&self, r: ByLocationRequest) -> ServiceResult<Vec<String>> {
    // Determine location
    let location: Location = match r.clone().location.ok_or(ServiceError::internal_error(
      "Nem sikerült a UPL lokációt dekódolni",
    ))? {
      by_location_request::Location::Stock(lid) => Location::Stock(lid),
      by_location_request::Location::Cart(lid) => Location::Cart(lid),
      by_location_request::Location::Delivery(lid) => Location::Delivery(lid),
      by_location_request::Location::Discard(lid) => Location::Discard(lid),
    };

    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| {
        let _upl = upl.unpack();
        _upl.location == location
      })
      .map(|upl| upl.unpack().id.clone())
      .collect::<Vec<String>>();

    Ok(res)
  }

  async fn set_best_before(&self, r: SetBestBeforeRequest) -> ServiceResult<UplObj> {
    // Process best_before
    let bbefore = match r.best_before.len() {
      x if x == 0 => None,
      _ => Some(
        DateTime::parse_from_rfc3339(&r.best_before)
          .map_err(|_| ServiceError::bad_request("A megadott dátum invalid!"))?
          .with_timezone(&Utc),
      ),
    };

    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .set_best_before(bbefore, r.created_by)
      .clone();

    Ok(res.into())
  }

  async fn split(&self, r: SplitRequest) -> ServiceResult<UplObj> {
    let new_upl = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .split(r.new_upl, r.piece, r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?;

    // Insert the new UPL
    self.upls.lock().await.insert(new_upl)?;

    // Select itself to send back as UplObj
    let res = self.upls.lock().await.find_id(&r.upl)?.unpack().clone();

    Ok(res.into())
  }

  async fn divide(&self, r: DivideRequest) -> ServiceResult<UplObj> {
    // Try to divide UPL
    let new_upl = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .divide(r.new_upl, r.requested_amount, r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Insert the new UPL into the UPL db
    self.upls.lock().await.insert(new_upl)?;

    // Find self and return as UplObj
    let res = self.upls.lock().await.find_id(&r.upl)?.unpack().clone();

    Ok(res.into())
  }

  async fn set_depreciation(&self, r: DepreciationRequest) -> ServiceResult<UplObj> {
    // Try to find UPL and set depreciation
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .set_depreciation(r.depreciation_id, r.depreciation_comment, r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Return self as UplObj
    Ok(res.into())
  }

  async fn remove_depreciation(&self, r: DepreciationRemoveRequest) -> ServiceResult<UplObj> {
    // Try find UPL and remove depreciation
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .remove_deprecation(r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Returns self as UplObj
    Ok(res.into())
  }

  async fn set_depreciation_price(&self, r: DepreciationPriceRequest) -> ServiceResult<UplObj> {
    // Try find UPL and set depreciation price
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .set_depreciation_price(Some(r.depreciation_net_price), r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Return self as UplObj
    Ok(res.into())
  }

  async fn remove_deprecation_price(
    &self,
    r: RemoveDeprecationPriceRequest,
  ) -> ServiceResult<UplObj> {
    // Try find UPL and remove depreciation price
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .set_depreciation_price(None, r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Return self as UplObj
    Ok(res.into())
  }

  async fn lock_to_cart(&self, r: CartLockRequest) -> ServiceResult<UplObj> {
    // Try to find UPL and lock to Cart(ID)
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .lock(upl::Lock::Cart(r.cart_id), r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Returns self as UplObj
    Ok(res.into())
  }

  async fn release_lock_from_cart(&self, r: CartUnlockRequest) -> ServiceResult<UplObj> {
    // Try to find UPL and unlock to Cart(ID)
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .unlock(upl::Lock::Cart(r.cart_id), r.created_by)
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();

    // Returns self as UplObj
    Ok(res.into())
  }

  async fn close_cart(&self, r: CloseCartRequest) -> ServiceResult<()> {
    // Try to find all the UPLs that have locked to
    // this given cart; and move them into that Cart Location.
    // This will automatically removes the lock::Cart(ID)
    self
      .upls
      .lock()
      .await
      .as_vec_mut()
      .into_iter()
      .for_each(|upl| {
        if upl.unpack().get_lock() == &upl::Lock::Cart(r.cart_id.clone()) {
          // todo! manage if result is error?
          let _ = upl
            .as_mut()
            .unpack()
            .move_upl(upl::Location::Cart(r.cart_id.clone()), r.created_by);
        }
      });

    // Collect upls to archive
    let upls_to_archive = self
      .upls
      .lock()
      .await
      .as_vec_mut()
      .into_iter()
      .filter(|upl| {
        if upl.unpack().get_location() == &upl::Location::Cart(r.cart_id.clone()) {
          return true;
        }
        false
      })
      .map(|upl| upl.unpack().clone())
      .collect::<Vec<upl::Upl>>();

    // Collect UPL IDs to remove them later
    let upl_ids = upls_to_archive
      .iter()
      .map(|u| u.get_id().clone())
      .collect::<Vec<String>>();

    // Archive UPLs
    for upl_to_arch in upls_to_archive {
      let _ = self.archive.lock().await.insert(upl_to_arch);
    }

    // Remove UPLs from active db
    for upl_id_to_remove in upl_ids {
      let _ = self.upls.lock().await.remove_pack(&upl_id_to_remove);
    }

    Ok(())
  }

  async fn set_sku_price(&self, r: SetSkuPriceRequest) -> ServiceResult<()> {
    // Try convert VAT
    let vat = upl::VAT::from_str(&r.vat).map_err(|e| ServiceError::bad_request(&e))?;
    // Check if prices valid
    if (r.net_price * vat) != r.gross_price {
      return Err(ServiceError::bad_request("A nettó * áfa != bruttó"));
    }
    // Reprice related UPLs
    self
      .upls
      .lock()
      .await
      .as_vec_mut()
      .into_iter()
      .for_each(|upl| {
        if upl.unpack().get_sku() == r.sku {
          let _ = upl.as_mut().unpack().set_price(r.net_price, vat);
          // TODO! LOG ERROR!
        }
      });
    // Return nothing
    Ok(())
  }

  async fn set_sku_divisible(&self, r: SetSkuDivisibleRequest) -> ServiceResult<()> {
    // Set related UPLs
    self
      .upls
      .lock()
      .await
      .as_vec_mut()
      .into_iter()
      .for_each(|upl| {
        if upl.unpack().get_sku() == r.sku {
          let _ = upl.as_mut().unpack().set_divisible(r.divisible);
          // TODO! ERROR LOG!
        }
      });
    // Return nothing
    Ok(())
  }

  // Try to open UPL
  async fn open_upl(&self, r: OpenUplRequest) -> ServiceResult<UplObj> {
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl_id)?
      .as_mut()
      .unpack()
      .open()
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();
    Ok(res.into())
  }

  // Try to close UPL
  async fn close_upl(&self, r: CloseUplRequest) -> ServiceResult<UplObj> {
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl_id)?
      .as_mut()
      .unpack()
      .close()
      .map_err(|e| ServiceError::bad_request(&e))?
      .clone();
    Ok(res.into())
  }

  // Try to merge back UPL
  async fn merge_back(&self, r: MergeRequest) -> ServiceResult<()> {
    // First find child UPL to merge
    let child_upl = self
      .upls
      .lock()
      .await
      .find_id(&r.upl_to_merge_back)?
      .unpack()
      .clone();

    // Check if UPL is a DerivedProduct
    match &child_upl.kind {
      upl::Kind::DerivedProduct {
        derived_from,
        derived_from_sku: _,
        amount: _,
      } => {
        // Find parent UPL and try to put merge back UPL
        let _ = self
          .upls
          .lock()
          .await
          .find_id_mut(&derived_from)?
          .as_mut()
          .unpack()
          .merge(child_upl.clone(), r.created_by)
          .map_err(|e| ServiceError::bad_request(&e))?
          .clone();

        // Remove child UPL as its merged
        self.upls.lock().await.remove_pack(child_upl.get_id())?;
      }
      _ => {
        return Err(ServiceError::bad_request(
          "A kért UPL nem kimért termék, így nem tehető vissza!",
        ))
      }
    }
    // Return nothing
    Ok(())
  }

  // Get SKU location info
  async fn get_location_info(&self, r: LocationInfoRequest) -> ServiceResult<LocationInfoResponse> {
    // Create empty response
    let mut res: LocationInfoResponse = LocationInfoResponse {
      sku: r.sku,
      stocks: HashMap::new(),
    };

    // Iterate over all the UPLs and collect stock info
    self.upls.lock().await.iter().for_each(|upl| {
      let _upl = upl.unpack();
      // If UPL has the required SKU
      if _upl.get_sku() == r.sku {
        match _upl.get_location() {
          Location::Stock(stock_id) => {
            let stock_info = res.stocks.entry(*stock_id).or_insert(StockInfo {
              total: 0,
              healthy: 0,
            });
            // Increment total count
            (*stock_info).total += _upl.get_upl_piece();
            // If its healthy then increment healthy count
            if _upl.is_available_healthy() {
              (*stock_info).healthy += _upl.get_upl_piece();
            }
          }
          _ => (),
        }
      }
    });

    Ok(res)
  }

  // Collect SKU location info in bulk
  async fn get_location_info_bulk(&self, r: Vec<u32>) -> ServiceResult<Vec<LocationInfoResponse>> {
    // Create empty response
    let mut res: HashMap<u32, LocationInfoResponse> = HashMap::new();

    // Iterate over all the UPLs and collect stock info
    self.upls.lock().await.iter().for_each(|upl| {
      let _upl = upl.unpack();
      // If UPL is int the required SKU list
      if r.contains(&_upl.get_sku()) {
        match _upl.get_location() {
          Location::Stock(stock_id) => {
            // Get location info for SKU or init it
            let location_info = res.entry(_upl.get_sku()).or_insert(LocationInfoResponse {
              sku: _upl.get_sku(),
              stocks: HashMap::new(),
            });

            // Get stock info or init it
            let stock_info = location_info.stocks.entry(*stock_id).or_insert(StockInfo {
              total: 0,
              healthy: 0,
            });

            // Increment total count
            (*stock_info).total += _upl.get_upl_piece();

            // If its healthy then increment healthy count
            if _upl.is_available_healthy() {
              (*stock_info).healthy += _upl.get_upl_piece();
            }
          }
          _ => (),
        }
      }
    });

    // Transform response from HashMap to Vec
    Ok(res.into_iter().map(|(_k, v)| v).collect())
  }
}

#[tonic::async_trait]
impl gzlib::proto::upl::upl_server::Upl for UplService {
  async fn create_new(&self, request: Request<UplNew>) -> Result<Response<UplObj>, Status> {
    let res = self.create_new(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn create_new_bulk(
    &self,
    request: Request<tonic::Streaming<UplNew>>,
  ) -> Result<Response<UplIds>, Status> {
    let mut stream = request.into_inner();

    let s = stream! {
        while let Some(new_upl) = stream.next().await {
          if let Ok(upl) = new_upl {
            if let Ok(res) = self.create_new(upl).await {
              yield res.id;
            }
          }
        }
    };

    pin_mut!(s);

    let mut upl_ids: Vec<String> = Vec::new();

    while let Some(value) = s.next().await {
      upl_ids.push(value);
    }

    Ok(Response::new(UplIds { upl_ids }))
  }

  type GetBulkStream = tokio::sync::mpsc::Receiver<Result<UplObj, Status>>;

  async fn get_bulk(
    &self,
    request: Request<BulkRequest>,
  ) -> Result<Response<Self::GetBulkStream>, Status> {
    // Create channel for stream response
    let (mut tx, rx) = tokio::sync::mpsc::channel(100);

    // Get resources as Vec<SourceObject>
    let res = self.get_bulk(request.into_inner()).await?;

    // Send the result items through the channel
    tokio::spawn(async move {
      for ots in res.into_iter() {
        tx.send(Ok(ots)).await.unwrap();
      }
    });

    // Send back the receiver
    Ok(Response::new(rx))
  }

  async fn get_by_id(&self, request: Request<ByIdRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.get_by_id(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn get_by_id_archive(
    &self,
    request: Request<ByIdRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.get_by_id_archive(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn get_by_sku(&self, request: Request<BySkuRequest>) -> Result<Response<UplIds>, Status> {
    let upl_ids = self.get_by_sku(request.into_inner()).await?;
    Ok(Response::new(UplIds { upl_ids }))
  }

  async fn get_by_sku_and_location(
    &self,
    request: Request<BySkuAndLocationRequest>,
  ) -> Result<Response<UplIds>, Status> {
    let upl_ids = self.get_by_sku_and_location(request.into_inner()).await?;
    Ok(Response::new(UplIds { upl_ids }))
  }

  async fn get_by_location(
    &self,
    request: Request<ByLocationRequest>,
  ) -> Result<Response<UplIds>, Status> {
    let upl_ids = self.get_by_location(request.into_inner()).await?;
    Ok(Response::new(UplIds { upl_ids }))
  }

  async fn set_best_before(
    &self,
    request: Request<SetBestBeforeRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.set_best_before(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn split(&self, request: Request<SplitRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.split(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn divide(&self, request: Request<DivideRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.divide(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn set_depreciation(
    &self,
    request: Request<DepreciationRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.set_depreciation(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn remove_depreciation(
    &self,
    request: Request<DepreciationRemoveRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.remove_depreciation(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn set_depreciation_price(
    &self,
    request: Request<DepreciationPriceRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.set_depreciation_price(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn remove_deprecation_price(
    &self,
    request: Request<RemoveDeprecationPriceRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.remove_deprecation_price(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn lock_to_cart(
    &self,
    request: Request<CartLockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.lock_to_cart(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn release_lock_from_cart(
    &self,
    request: Request<CartUnlockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.release_lock_from_cart(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn close_cart(&self, request: Request<CloseCartRequest>) -> Result<Response<()>, Status> {
    let _ = self.close_cart(request.into_inner()).await?;
    Ok(Response::new(()))
  }

  async fn set_sku_price(
    &self,
    request: Request<SetSkuPriceRequest>,
  ) -> Result<Response<()>, Status> {
    let _ = self.set_sku_price(request.into_inner()).await?;
    Ok(Response::new(()))
  }

  async fn set_sku_divisible(
    &self,
    request: Request<SetSkuDivisibleRequest>,
  ) -> Result<Response<()>, Status> {
    let _ = self.set_sku_divisible(request.into_inner()).await?;
    Ok(Response::new(()))
  }

  async fn open_upl(&self, request: Request<OpenUplRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.open_upl(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn close_upl(&self, request: Request<CloseUplRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.close_upl(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn merge_back(&self, request: Request<MergeRequest>) -> Result<Response<()>, Status> {
    let _ = self.merge_back(request.into_inner()).await?;
    Ok(Response::new(()))
  }

  async fn get_location_info(
    &self,
    request: Request<LocationInfoRequest>,
  ) -> Result<Response<LocationInfoResponse>, Status> {
    let res = self.get_location_info(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  type GetLocationInfoBulkStream =
    tokio::sync::mpsc::Receiver<Result<LocationInfoResponse, Status>>;

  async fn get_location_info_bulk(
    &self,
    request: Request<LocationInfoBulkRequest>,
  ) -> Result<Response<Self::GetLocationInfoBulkStream>, Status> {
    // Create channel for stream response
    let (mut tx, rx) = tokio::sync::mpsc::channel(100);

    // Get resources as Vec<SourceObject>
    let res = self
      .get_location_info_bulk(request.into_inner().sku)
      .await?;

    // Send the result items through the channel
    tokio::spawn(async move {
      for ots in res.into_iter() {
        tx.send(Ok(ots)).await.unwrap();
      }
    });

    // Send back the receiver
    Ok(Response::new(rx))
  }
}

#[tokio::main]
async fn main() -> prelude::ServiceResult<()> {
  // Init UPL DB
  let upl_db: VecPack<upl::Upl> =
    VecPack::load_or_init(PathBuf::from("data/upls")).expect("Error while loading UPL database");

  // Init UPL DB
  let archive_db: VecPack<upl::Upl> = VecPack::load_or_init(PathBuf::from("data/upl_archive"))
    .expect("Error while loading UPL archive database");

  let upl_service = UplService::init(upl_db, archive_db);

  let addr = env::var("SERVICE_ADDR_UPL")
    .unwrap_or("[::1]:50064".into())
    .parse()
    .unwrap();

  // Create shutdown channel
  let (tx, rx) = oneshot::channel();

  // Spawn the server into a runtime
  tokio::task::spawn(async move {
    Server::builder()
      .add_service(UplServer::new(upl_service))
      .serve_with_shutdown(addr, async { rx.await.unwrap() })
      .await
  });

  tokio::signal::ctrl_c().await.unwrap();

  println!("SIGINT");

  // Send shutdown signal after SIGINT received
  let _ = tx.send(());

  Ok(())
}
