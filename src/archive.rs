use crate::Upl;
use std::fs::create_dir_all;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub enum ArchiveError {}

pub struct ArchiveStore {
  path: PathBuf,
}

impl ArchiveStore {
  pub fn init(path: PathBuf) -> ArchiveStore {
    // 1. Check if path exist
    if !path.exists() {
      // 2. Create path if does not exist
      // we use expect as its error should stop the program at the begenning
      create_dir_all(&path).expect("Error while creating UplIndex path tree! (It did not exist");
    }
    Self { path }
  }
  pub fn add(&self, upl: Upl) -> Result<(), ArchiveError> {
    // 1. Generate UPL Archive object path
    // 2. Check if path exist (should be impossible)
    // 3. Create Archive Object file in FS
    // 4. Convert UPL to ArchiveObject
    // 5. Serialize ArchiveObject
    // 6. Save object to Archive Object file
    todo!();
  }
  pub fn restore(&self, upl_id: u32) -> Result<Upl, ArchiveError> {
    // 1. Check archive file exist
    // 2. Try load
    // 3. Try create UPL Object
    // 4. Remove Archive object file from FS
    todo!();
  }
}

#[derive(Serialize, Deserialize)]
pub struct ArchiveObject {
  upl: u32,
}

impl ArchiveObject {}
