use std::path::PathBuf;

mod prelude;
mod upl;

use futures::lock::Mutex;
use packman::*;
use upl::Upl;

// mod reservation;

struct UplService {
  upls: Mutex<VecPack<Upl>>,
}

impl UplService {
  fn init(upls: VecPack<Upl>) -> Self {
    Self {
      upls: Mutex::new(upls),
    }
  }
}

fn main() {
  // Init UPL DB
  let upls: VecPack<Upl> =
    VecPack::load_or_init(PathBuf::from("data/upls")).expect("Error while loading UPL database");
}
