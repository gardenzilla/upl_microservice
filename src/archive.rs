use crate::Upl;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::path::PathBuf;

// Determine UPL index path parts from UPL
// This kind of partitioning enable us to store safely
// millions of UPLs without crashing the FS.
// Maximum 1_000 folder per folder and maximum 1_000 index file
// per folder.
// returns (million value, thousand value, hundreds value)
#[inline]
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
  // When the UPL has successfully sold
  Sold,
  //
  Missing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Location {
  Stock(u32),
  Cart(u32),
}

impl Default for Location {
  fn default() -> Self {
    Self::Stock(0)
  }
}

/// UPL Kind
/// Represents the UPL phisical appearance
/// Can be
///   SKU => UPL is an un-opened SKU
///   BulkSku => UPL is a bulk of un-opened SKUs
///   OpenedSku =>  UPL represents a SKU that has opened and some of its quantity
///                 has already taken out.
///   DerivedProduct => its an opened SKU, but the moved out part, and its moved to another
///                     package. Based on its appearance we cannot tell which SKU its related
///                     but we can tell, which product it is.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Kind {
  // UPL representing a single SKU
  // Has its own UPL ID
  Sku {
    product_id: u32,
    sku: u32,
  },
  // Muliple un-opened SKU in a bulk package
  // sub UPLs cannot have a UPL ID yet, but we all of them
  // share the same UPL attributes. So when we split this bulk
  // package we create the UPLs by cloning its attributes.
  BulkSku {
    product_id: u32,
    sku: u32,
  },
  // An opened sku
  // An original sku has expanded,
  // and a part of it is already out of it.
  OpenedSku {
    product_id: u32,
    sku: u32,
    amount: u32,
  },
  // Piece of product that
  // derives from an opened sku
  DerivedProduct {
    // Related product ID
    product_id: u32,
    // Derived SKU
    // Can be only Sku, or OpenedSku
    derived_from: u32,
    // Amount in the products unit
    amount: u32,
  },
}

impl Default for Kind {
  fn default() -> Self {
    Self::Sku {
      product_id: 0,
      sku: 0,
    }
  }
}

/// Lock kinds
/// Cart lock means the given UPL is locked to a specific Cart
/// so it cannot move away, as its under a sales process.
/// None means there is no lock, so the UPL can be moved away.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Lock {
  // Using when a UPL is in a Cart
  Cart(u32),
  // Using when UPL is under an inventory process
  // and missing
  // todo!: Should have a better naming
  Inventory(u32),
  // UPL has no lock
  // it can be updated and moved freely
  None,
}

#[derive(Serialize, Deserialize)]
pub struct ArchiveObject {
  reason: ArchiveReason,
  upl_id: u32,
  // UPL Kind
  // Single or Bulk(u32)
  // Single means its a single UPL,
  // Bulk means its a collection of UPLs under a single UPL ID
  // e.g. a pallet flower soil (50)
  kind: Kind,
  // * Procurement
  procurement_id: u32,
  // Net wholesale price in which
  // this item was purchased by us
  procurement_net_price: f32,
  // Current UPL location
  location: Location, // todo? this way?
  // todo! Not NOW!
  // todo! Implement => location_history: Vec<Location>,
  // --
  // If the product is injured
  // it should be scraped. This field
  // contains the related scrap id
  scrap_id: Option<i32>, // TODO: scrap_price_log?
  // Related scrap comment
  // if there any
  // From the sku scrap comment from the
  // related scrap record
  scrap_comment: Option<String>,
  // Related scrap price
  // if there any.
  // Can set if there is related scrap_id
  scrap_retail_net_price: Option<f32>,
  // Best before date
  // Only for perishable goods.
  // Optional, but when we have one, we use
  // DateTime<Utc>
  best_before: Option<DateTime<Utc>>,
  // Product quantity
  // It contains Simple or Complex quantity
  // Or when a Simple product - wich is divisible -
  // is divided, it contains the remained quantity.
  // Inherited from Product(service), but after
  // bacome Partial(u32), it's going to be managed
  // here without responding the related Product changes.
  // --
  // Only some, if Sku can be devided, and its unopened.
  // Once its opened, this amount will be none, and its
  // value is moved to its kind component
  // This value represents the SKU original divisible quantity
  divisible_amount: Option<u32>,
  // Lock enum
  // When a UPL is locked by any reason,
  // that UPL cannot be updated.
  // ~ Only ~ the lock owner can unlock
  lock: Lock,
  // Userid who created
  created_by: String,
  // Utc datetime when this object
  // created
  date_created: DateTime<Utc>,
}

impl ArchiveObject {}

pub struct ArchiveStore {
  path: PathBuf,
}

impl ArchiveStore {
  pub fn init(path: PathBuf) -> ArchiveStore {
    // 1. Check if path exist
    if !path.exists() {
      // 2. Create path if does not exist
      // we use expect as its error should stop the program at the beginning
      create_dir_all(&path).expect("Error while creating UplIndex path tree! (It did not exist");
    }
    Self { path }
  }
  // Only when a UPL has any Lock
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
  // Restore UPL
  // Only a RECEIVER LOCK PROVIDER CAN request a restore process
  // e.g.:  - When a PURCHASE has rolled back, and we take back a product,
  //          then that UPL might be restored into that active and opened Cart
  //        - When a missing UPL was found, we can create a NEW INVENTORY LOG,
  //          and add a FOUND UPL. Then the UPL is going to be restored there.
  pub fn restore(&self, upl_id: u32) -> Result<Upl, ArchiveError> {
    // 1. Check archive file exist
    // 2. Try load
    // 3. Try create UPL Object
    // 4. Set the new Lock type and ID
    // 5. Remove Archive object file from FS
    // 6. Return UPL
    todo!();
  }
}
