use crate::upl::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Debug)]
pub enum IndexError {
  NotFound,
  WrongId,
  FileReadError,
  FileDeserializeError,
  FileSerializeError,
  AlreadyExist,
  InternalError(String),
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct IndexObject {
  // Base ID
  pub base_id: u32,
  // ID with checksum characters
  pub upl: u32,
  // Related product ID
  pub product: u32,
  // Related SKU ID
  pub sku: Option<u32>,
  // Created at
  pub created_at_unix_ts_utc: i64, // unix epoch
}

impl IndexObject {
  pub fn new(base_id: u32, upl: u32, product: u32, sku: Option<u32>) -> Self {
    Self {
      base_id,
      upl,
      product,
      sku,
      created_at_unix_ts_utc: Utc::now().timestamp(),
    }
  }
}

impl UplIndex {
  pub fn init(path: PathBuf) -> Self {
    // 1. Check if path exist
    if !path.exists() {
      // 2. Create path if does not exist
      // we use expect as its error should stop the program at the begenning
      create_dir_all(&path).expect("Error while creating UplIndex path tree! (It did not exist");
    }
    Self { path }
  }

  /// Get UPLIndex object
  fn get(&self, id: u32) -> Result<IndexObject, IndexError> {
    // 1. Check ID checksum (Validate it)
    // todo! Implement ID checksum validation or do we have it before?
    ();
    // 2. Get base ID
    // This means we cut the last two characters
    // This means divide by 100
    let base = id / 100;
    let (parent, child, _) = get_path(base);

    let file_path = self
      .path
      .join(parent.to_string())
      .join(child.to_string())
      .join(format!("{}.IndexObject", id));

    // If index file does not exist
    // return error
    if !file_path.exists() {
      return Err(IndexError::NotFound);
    }

    // 3. Try load index file
    // Read file content into file_str
    let file_str = std::fs::read_to_string(&file_path).map_err(|_| IndexError::FileReadError)?;

    // 4. Try deserialize index file
    //    and return the index file or error
    Ok(
      serde_yaml::from_str::<IndexObject>(&file_str)
        .map_err(|_| IndexError::FileDeserializeError)?,
    )
  }

  /// Add UPL as a UPL Index
  fn add(&self, upl: &Upl) -> Result<(), IndexError> {
    // 1. Get base ID from UplId
    let base = upl.id / 100;

    // 2. Create index file path object
    let (parent, child, _) = get_path(base);
    let folder_path = self.path.join(parent.to_string()).join(child.to_string());
    let file_path = folder_path.join(format!("{}.IndexObject", upl.id));

    // 3. Check if the index file already exist
    if file_path.exists() {
      return Err(IndexError::AlreadyExist);
    }

    // 4. Check if folder path exist
    //    and create it all if does not
    if !folder_path.exists() {
      std::fs::create_dir_all(&folder_path).map_err(|_| {
        IndexError::InternalError(format!(
          "A megadott path-t nem lehet létrehozni! {:?}",
          &folder_path
        ))
      })?;
    }

    // 3. Create index object from the given UPL
    let index_object =
      IndexObject::new(base, upl.id, *upl.get_product_id(), upl.get_sku().cloned());

    // 4. Create index file
    let mut index_file = std::fs::File::create(&file_path).map_err(|_| {
      IndexError::InternalError(format!("Error while creating index file: {:?}", &file_path))
    })?;

    // 5. Try serialize index object and try save it
    //    into the index file
    serde_yaml::to_writer(&mut index_file, &index_object)
      .map_err(|_| IndexError::FileSerializeError)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::sync::Once;

  static INIT: Once = Once::new();

  fn clean() {
    INIT.call_once(|| {
      let path = PathBuf::from("data/test/index");
      std::fs::remove_dir_all(path).expect("Failed to clean up test index directory");
    });
  }

  #[test]
  fn test_create() {
    clean();
    let mut upl = Upl::default();
    upl.id = 101598512;

    let index = UplIndex::init(PathBuf::from("data/test/index"));
    assert_eq!(index.add(&upl).is_ok(), true);
  }

  #[test]
  fn test_get() {
    let id = 101598512;
    let index = UplIndex::init(PathBuf::from("data/test/index"));
    let i = index.get(id);
    assert_eq!(i.is_ok(), true, "loaded file has success deser {}", id);
    assert_eq!(
      i.unwrap().upl,
      id,
      "deserialized index file has a wrong UPL {}",
      id
    );
  }
}
