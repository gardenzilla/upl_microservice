use std::{fs::create_dir_all, path::PathBuf};

use crate::upl::Upl;

pub enum IndexError {
  NotFound,
  WrongId,
}

pub struct UplIndex {
  path: PathBuf,
}

// Determine UPL index path parts from UPL
// This kind of partinioning enable us to store safily
// millions of UPLs without crashing the FS.
// Maximum 1_000 folder per folder and maximum 1_000 index file
// per folder.
// returns (million value, thousand value, hunders value)
fn get_path(u: u32) -> (u32, u32, u32) {
  (u / 1_000_000, u % 1_000_000 / 1000, u % 1000)
}

struct UIndex {
  index_id: u32,
  upl: u32,
  product: u32,
  sku: u32,
  created_at_epoch_utc: u32, // unix epoch
}

impl UplIndex {
  fn init(path: PathBuf) -> Self {
    // 1. Check if path exist
    if !path.exists() {
      // 2. Create path if does not exist
      // we use expect as its error should stop the program at the begenning
      create_dir_all(&path).expect("Error while creating UplIndex path tree! (It did not exist");
    }
    Self { path }
  }
  fn get(id: u32) -> Result<(), IndexError> {
    // 1. Check ID checksum
    // 2. Get base ID
    // 3. Try load index file
    // 4. Return the index file or error
    todo!("Implement UplIndex GET")
  }
  fn add(upl: &Upl) -> Result<(), IndexError> {
    // 1. Get base ID from UplId
    // 2. Create index object from the given UPL
    // 3. Calculate path from the base ID (1000 index files per folder max)
    // 3. Create index file
    // 4. Try serialize index object and try save it
    //    into the index file
    todo!("Implement UplIndex ADD")
  }
}
