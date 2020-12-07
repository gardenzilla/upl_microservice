use crate::Upl;
use std::fs::create_dir_all;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// Determine UPL index path parts from UPL
// This kind of partinioning enable us to store safily
// millions of UPLs without crashing the FS.
// Maximum 1_000 folder per folder and maximum 1_000 index file
// per folder.
// returns (million value, thousand value, hunders value)
fn get_path(u: u32) -> (u32, u32, u32) {
  (u / 1_000_000, u % 1_000_000 / 1000, u % 1000)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ArchiveError {
  InternalError(String),
  AlreadyExist(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ArchiveReason {
  Sold,
  Scrapping,
}

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
    let base = upl.id / 100;
    let (parent, child, _) = get_path(base);
    let folder_path = self.path.join(parent.to_string()).join(child.to_string());
    let file_path = folder_path.join(format!("{}.uarch", upl.id));

    // Create folder path all
    // if not yet exist
    if !folder_path.exists() {
      create_dir_all(&folder_path).map_err(|e| ArchiveError::InternalError(e.to_string()))?;
    }

    // 2. Check if path exist (should be impossible)
    // If archive file does exist
    // return error
    if file_path.exists() {
      return Err(ArchiveError::AlreadyExist(format!(
        "A megadott ID már archiválva van! {}",
        upl.id
      )));
    }

    // 3. Set Archive history event

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
