use std::path::PathBuf;

use packman::*;
use reservation::Reservation;
use serde::Serialize;

mod archive;
mod id;
mod index;
mod prelude;
mod reservation;
mod upl;

pub use id::*;
use upl::Upl;

struct UplStore {
  // Store all the managed (in-stock) UPLs
  upls: VecPack<Upl>,
  // Store next UPL Id
  // to build UplId using checksum
  next_id: Pack<u32>,
}

impl UplStore {
  fn init(upls: VecPack<Upl>, next_id: Pack<u32>) -> Self {
    UplStore { upls, next_id }
  }
}

struct UplService {
  // Create | Get
  index: (),
  // AddReservation | AddUpl | Increase | Descrease | ClearCart
  reservation: Pack<Vec<Reservation>>,
  // Create | Move | ..
  store: UplStore,
  // New | Restore | Get
  archive: (),
}

impl UplService {
  fn init(index: (), reservation: Pack<Vec<Reservation>>, store: UplStore, archive: ()) -> Self {
    Self {
      index,
      reservation,
      store,
      archive,
    }
  }
}

fn main() {
  // Init UPL Index
  let upl_index = ();

  // Init Reservation DB
  let reservation: Pack<Vec<Reservation>> =
    Pack::load_or_init(PathBuf::from("data"), "reservation")
      .expect("Error while loading reservation database");

  // Init UPL vecpack db
  let upls: VecPack<Upl> =
    VecPack::load_or_init(PathBuf::from("data/upls")).expect("Error while loading UPL database");

  // Init UPL next ID db
  let next_id: Pack<u32> = Pack::load_or_init(PathBuf::from("data"), "upl_next_id")
    .expect("Error while loading UPL next id db");

  // Init UPL store based on the pre-loaded DBs
  let upl_store: UplStore = UplStore::init(upls, next_id);

  // Init UPL Archive
  // All the sold UPLs are stored here
  let upl_archive = ();

  // Init UplService
  let upl_service: UplService = UplService::init(upl_index, reservation, upl_store, upl_archive);

  // RPC code goes here
}
