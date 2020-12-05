use crate::id;
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait _Upl {
  fn get_piece(&self) -> u32; // Should be a better name
  fn lock(&mut self, cart_id: u32) -> Result<(), String>;
  fn unlock(&mut self, cart_id: u32) -> Result<(), String>;
  fn move_upl(&mut self, from: u32, to: u32, upl_id: u32) -> Result<(), String>;
}

pub enum Location {
  Cart(u32),
}

pub enum Kind {
  Sku {
    product_id: u32,
    sku: u32,
  },
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
    product_id: u32,
    amount: u32,
  },
}

pub enum Lock {
  CartLock { cart_id: u32 },
  None,
}

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
  pub fn is_dirty(&self) -> bool {
    self.dirty
  }
  pub fn can_sell(&self) -> bool {
    // Can sell if
    // there is VAT set, and there is retail price or scrap retail price given
    self.vat.is_some() && (self.scrap_retail_net_price.is_some() | self.retail_net_price.is_some())
  }
}

mod v2 {
  struct UplData {}

  enum Upl {
    Single(UplData),
    Pallet(UplData, u32),
  }

  impl Upl {
    fn get_amount(&self) -> u32 {
      match self {
        Upl::Single(_) => 1,
        Upl::Pallet(_, a) => *a,
      }
    }
  }

  enum Lock {
    Cart(u32),
  }

  enum Location {
    Stock(u32),
    // Delivery(u32),
    Purchase(u32),
  }

  trait UplStore {
    fn move_upl(&mut self, upl_id: u32, from: u32, to: u32) -> Result<&Upl, ()>;
    fn lock_cart(&mut self, upl_id: u32, cart_id: u32) -> Result<&Upl, ()>;
    fn unlock(&mut self, upl_id: u32) -> Result<&Upl, ()>;
    fn get(&self, upl_id: u32) -> Result<&Upl, ()>;
    fn get_by_location(&self, location_id: u32, upl_id: u32) -> Result<Vec<&Upl>, ()>;
  }

  fn _main() {
    let store: Vec<Upl> = Vec::new();
  }
}

// SKU => Promise(CartItem, Piece) / Real(Vec<UplId>)

// message Upl {
//   string id = 1;
//   ..;
//   string kind = b;
// }
