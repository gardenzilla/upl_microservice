use std::path::PathBuf;

use packman::*;
use reservation::Reservation;
use serde::Serialize;

pub mod id;
pub mod prelude;
pub mod reservation;
pub mod upl;

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
  index: (),
  reservation: Pack<Vec<Reservation>>,
  store: UplStore,
  archive: (),
}

impl UplService {}

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
}
