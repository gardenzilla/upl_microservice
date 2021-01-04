use gzlib::proto::upl::upl_server::*;
use gzlib::proto::upl::*;
use packman::*;
use std::{env, path::PathBuf};
use tokio::sync::{oneshot, Mutex};
use tonic::{transport::Server, Request, Response, Status};

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
    todo!()
  }
}

#[tonic::async_trait]
impl Upl for UplService {
  async fn create_new(&self, request: Request<UplNew>) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn get_by_id(&self, request: Request<ByIdRequest>) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  type GetBySkuStream;

  async fn get_by_sku(
    &self,
    request: Request<BySkuRequest>,
  ) -> Result<Response<Self::GetBySkuStream>, Status> {
    todo!()
  }

  type GetBySkuAndLocationStream;

  async fn get_by_sku_and_location(
    &self,
    request: Request<BySkuAndLocationRequest>,
  ) -> Result<Response<Self::GetBySkuAndLocationStream>, Status> {
    todo!()
  }

  type GetByLocationStream;

  async fn get_by_location(
    &self,
    request: Request<ByLocationRequest>,
  ) -> Result<Response<Self::GetByLocationStream>, Status> {
    todo!()
  }

  async fn set_best_before(
    &self,
    request: Request<SetBestBeforeRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn split(&self, request: Request<SplitRequest>) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn divide(&self, request: Request<DivideRequest>) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn set_depreciation(
    &self,
    request: Request<DepreciationRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn remove_depreciation(
    &self,
    request: Request<DepreciationRemovePriceRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn set_depreciation_price(
    &self,
    request: Request<DepreciationPriceRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn lock_to_cart(
    &self,
    request: Request<CartLockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn lock_to_inventory(
    &self,
    request: Request<InventoryLockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn release_lock_from_cart(
    &self,
    request: Request<CartUnlockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn release_lock_from_inventory(
    &self,
    request: Request<InventoryUnlockRequest>,
  ) -> Result<Response<UplObj>, Status> {
    todo!()
  }

  async fn close_cart(&self, request: Request<CloseCartRequest>) -> Result<Response<E>, Status> {
    todo!()
  }

  async fn close_inventory(
    &self,
    request: Request<CloseInventoryRequest>,
  ) -> Result<Response<E>, Status> {
    todo!()
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
