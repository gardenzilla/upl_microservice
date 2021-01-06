use chrono::{DateTime, Utc};
use futures_util::stream::StreamExt;
use gzlib::proto::upl::upl_server::*;
use gzlib::proto::upl::*;
use packman::*;
use prelude::*;
use std::{env, path::PathBuf};
use tokio::sync::{oneshot, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use upl::{Location, UplMethods};

mod prelude;
mod upl;

// mod reservation;

struct UplService {
  upls: Mutex<VecPack<upl::Upl>>,
}

impl UplService {
  fn init(upls: VecPack<upl::Upl>) -> Self {
    Self {
      upls: Mutex::new(upls),
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

    // Create new UPL Object
    let new_upl = upl::Upl::new(
      r.upl_id,
      r.product_id,
      r.sku,
      r.upl_piece,
      r.procurement_id,
      r.procurement_net_price,
      upl::Location::Stock(r.stock_id),
      best_before,
      match r.divisible_amount {
        x if x == 0 => None,
        _ => Some(r.divisible_amount),
      },
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

  async fn get_by_sku(&self, r: BySkuRequest) -> ServiceResult<Vec<String>> {
    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| upl.unpack().get_sku() == Some(r.sku))
      .map(|upl| upl.unpack().id.clone())
      .collect::<Vec<String>>();

    Ok(res)
  }

  async fn get_by_sku_and_location(
    &self,
    r: BySkuAndLocationRequest,
  ) -> ServiceResult<Vec<String>> {
    let l: LocationKind = match LocationKind::from_i32(r.location_kind) {
      Some(l) => l,
      None => {
        return Err(ServiceError::internal_error(
          "A megadott UPL lokációs nem tudtuk azonosítani!",
        ))
      }
    };

    // Determine location
    let location: Location = match l {
      LocationKind::Cart => Location::Cart(r.location_id),
      LocationKind::Stock => Location::Stock(r.location_id),
      LocationKind::Delivery => Location::Delivery(r.location_id),
      LocationKind::Discard => Location::Discard(r.location_id),
    };

    let res = self
      .upls
      .lock()
      .await
      .iter()
      .filter(|upl| {
        let _upl = upl.unpack();
        _upl.get_sku() == Some(r.sku) && _upl.location == location
      })
      .map(|upl| upl.unpack().id.clone())
      .collect::<Vec<String>>();

    Ok(res)
  }

  async fn get_by_location(&self, r: ByLocationRequest) -> ServiceResult<Vec<String>> {
    let l: LocationKind = match LocationKind::from_i32(r.location_kind) {
      Some(l) => l,
      None => {
        return Err(ServiceError::internal_error(
          "A megadott UPL lokációs nem tudtuk azonosítani!",
        ))
      }
    };

    // Determine location
    let location: Location = match l {
      LocationKind::Cart => Location::Cart(r.location_id),
      LocationKind::Stock => Location::Stock(r.location_id),
      LocationKind::Delivery => Location::Delivery(r.location_id),
      LocationKind::Discard => Location::Discard(r.location_id),
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
      .split(r.new_upl, r.created_by)
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

  async fn lock_to_inventory(&self, r: InventoryLockRequest) -> ServiceResult<UplObj> {
    // Try to find UPL and lock to Cart(ID)
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .lock(upl::Lock::Inventory(r.inventory_id), r.created_by)
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

  async fn release_lock_from_inventory(&self, r: InventoryUnlockRequest) -> ServiceResult<UplObj> {
    // Try to find UPL and unlock to Cart(ID)
    let res = self
      .upls
      .lock()
      .await
      .find_id_mut(&r.upl)?
      .as_mut()
      .unpack()
      .unlock(upl::Lock::Inventory(r.inventory_id), r.created_by)
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
        if upl.unpack().get_lock() == &upl::Lock::Cart(r.cart_id) {
          // todo! manage if result is error?
          let _ = upl
            .as_mut()
            .unpack()
            .move_upl(upl::Location::Cart(r.cart_id), r.created_by);
        }
      });
    Ok(())
  }

  async fn close_inventory(&self, r: CloseInventoryRequest) -> ServiceResult<()> {
    for upl in self.upls.lock().await.as_vec_mut() {
      if upl.get_lock() == &upl::Lock::Inventory(r.inventory_id) {
        upl.as_mut().unpack().unlock_forced();
      }
    }
    Ok(())
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
  ) -> Result<Response<()>, Status> {
    let mut stream = request.into_inner();

    let _ = async_stream::try_stream! {
        while let Some(new_upl) = stream.next().await {
          if let Ok(upl) = new_upl {
            let _ = self.create_new(upl).await;
          }
        }
    };

    Ok(Response::new(()))
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
    for sobject in res {
      tx.send(Ok(sobject))
        .await
        .map_err(|_| Status::internal("Error while sending sources over channel"))?;
    }

    // Send back the receiver
    Ok(Response::new(rx))
  }

  async fn get_by_id(&self, request: Request<ByIdRequest>) -> Result<Response<UplObj>, Status> {
    let res = self.get_by_id(request.into_inner()).await?;
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

  async fn lock_to_inventory(
    &self,
    request: Request<InventoryLockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.lock_to_inventory(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn release_lock_from_cart(
    &self,
    request: Request<CartUnlockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self.release_lock_from_cart(request.into_inner()).await?;
    Ok(Response::new(res))
  }

  async fn release_lock_from_inventory(
    &self,
    request: Request<InventoryUnlockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    let res = self
      .release_lock_from_inventory(request.into_inner())
      .await?;
    Ok(Response::new(res))
  }

  async fn close_cart(&self, request: Request<CloseCartRequest>) -> Result<Response<()>, Status> {
    let _ = self.close_cart(request.into_inner()).await?;
    Ok(Response::new(()))
  }

  async fn close_inventory(
    &self,
    request: Request<CloseInventoryRequest>,
  ) -> Result<Response<()>, Status> {
    let _ = self.close_inventory(request.into_inner()).await?;
    Ok(Response::new(()))
  }
}

#[tokio::main]
async fn main() -> prelude::ServiceResult<()> {
  // Init UPL DB
  let db: VecPack<upl::Upl> =
    VecPack::load_or_init(PathBuf::from("data/upls")).expect("Error while loading UPL database");

  let upl_service = UplService::init(db);

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
