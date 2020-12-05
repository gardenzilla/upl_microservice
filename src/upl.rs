use crate::id;
use crate::prelude::*;
use chrono::prelude::*;
use packman::VecPackMember;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait _Upl {
  fn get_piece(&self) -> u32; // Should be a better name
  fn lock(&mut self, cart_id: u32) -> Result<(), String>;
  fn unlock(&mut self, cart_id: u32) -> Result<(), String>;
  fn move_upl(&mut self, from: u32, to: u32, upl_id: u32) -> Result<(), String>;
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
  Cart(u32),
  None,
}

// Default implementation for Lock
impl Default for Lock {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Upl {
  // Unique UPL ID
  // i32 for the better inter
  // service communication
  id: id::UplId,
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

impl Default for Upl {
  fn default() -> Self {
    Self {
      id: id::UplId::default(),
      kind: Kind::default(),
      procurement_id: 0,
      procurement_net_price: 0.0,
      location: Location::default(),
      scrap_id: None,
      scrap_comment: None,
      scrap_retail_net_price: None,
      best_before: None,
      divisible_amount: None,
      lock: Lock::default(),
      created_by: "".into(),
      date_created: Utc::now(),
    }
  }
}

impl Upl {
  /// Check whether a UPL is locked,
  /// or not.
  pub fn is_locked(&self) -> bool {
    match self.lock {
      Lock::None => false,
      _ => true,
    }
  }
  /// Get UPL lock None if no lock
  /// otherwise the given lock kind
  /// variant
  pub fn get_lock(&self) -> &Lock {
    &self.lock
  }
  pub fn is_scraped(&self) -> bool {
    match self.scrap_id {
      Some(_) => true,
      _ => false,
    }
  }
}

impl VecPackMember for Upl {
  type Out = u32;

  fn get_id(&self) -> &Self::Out {
    &self.id
  }
}
