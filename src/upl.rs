use crate::id;
use crate::prelude::*;
use chrono::prelude::*;
use packman::VecPackMember;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// UPL method declarations
pub trait UplMethods {
  /// Create a new UPL Object
  /// by the given details
  /// Should be used only by the procurement service
  fn new() -> Upl;
  /// Get UPL ID ref
  fn get_id(&self) -> &u32;
  /// Get UPL Kind ref
  fn get_kind(&self) -> &Kind;
  /// Get UPL procurement ID ref
  fn get_procurement_id(&self) -> &u32;
  /// Get UPL procurement net price ref
  fn get_procurement_net_price(&self) -> &u32;
  /// Check whether UPL can move to a different location
  /// depends on its acquired Lock kind
  fn can_move(&self, to: &Location) -> bool;
  /// Get current location ref
  fn get_location(&self) -> &Location;
  /// Try move UPL from location A to location B
  fn move_upl(&mut self, from: Location, to: Location) -> Result<(), String>;
  /// Check whether UPL has a lock or none
  fn has_lock(&self) -> bool;
  /// Get UPL lock ref
  fn get_lock(&self) -> &Lock;
  /// Try to lock UPL by a given Lock
  fn lock(&mut self, lock: Lock) -> Result<(), String>;
  /// Try to unlock UPL
  fn unlock(&mut self) -> Result<(), String>;
  /// Check if the UPL is depreciated
  /// This can mean a damaged package, or anything the might
  /// lower the UPL value, but it can still be sold.
  fn is_depreciated(&self) -> bool;
  /// Get depreciation ID if there is any
  fn get_depreciation_id(&self) -> Option<i32>;
  /// Get depreciation comment if there is any
  fn get_depreciation_comment(&self) -> Option<&String>;
  /// Get depreciation price if there is any
  fn get_depreciation_price(&self) -> Option<u32>;
  /// Get best before date if there is any
  fn get_best_before(&self) -> Option<NaiveDate>;
  /// Update UPL best_before date
  /// for any reason
  /// Should be private and used only from the inventory service
  fn update_best_before(&mut self, best_before: NaiveDate) -> &Upl;
  /// Check whether the UPL is an un-opened original one or not
  fn is_original(&self) -> bool;
  /// Check if its a bulk UPL
  fn is_bulk(&self) -> bool;
  /// Returns how many UPLs are packed inside this UPL
  /// As default its always 1
  /// but when a UPL is a bulk UPL, then multiple UPLs are
  /// packed into one bulk package
  fn get_upl_piece(&self) -> u32;
  /// Split one UPL from the bulk ones
  /// ----------
  /// IMPORTANT!
  /// ----------
  /// in a higher lever you must save the split UPL in the UPL store
  fn split(&mut self, new_upl_id: u32) -> Result<Upl, ()>;
  /// Split multiple UPLs from the bulk ones
  /// ----------
  /// IMPORTANT!
  /// ----------
  /// in a higher lever you must save the split UPL in the UPL store
  fn split_bulk(&mut self, new_upl_ids: Vec<u32>) -> Result<Vec<Upl>, ()>;
  /// Divide a divisible UPL into two UPLs
  /// If the UPL is a divisible Sku, then it will become an OpenedSku
  /// and the resulted new Upl will be a DerivedProduct
  /// ----------
  /// IMPORTANT!
  /// ----------
  /// in a higher lever you must save the split UPL in the UPL store
  fn divide(&mut self, new_upl_id: u32, requested_amount: u32) -> Result<Upl, ()>;
  /// Try to merge a source and a derived UPL into together
  /// When for any reason we want to put back a derived UPL into
  /// its ancestor
  fn merge(&mut self, upl: Upl) -> Result<&Upl, ()>;
  /// Check whether this UPL is divisible or not
  fn is_divisible(&self) -> bool;
  /// If the UPL is divisible,
  /// returns the remaining amount that can be divide
  fn get_divisible_amount(&self) -> Option<u32>;
  /// Get UPL history
  fn get_history(&self) -> &Vec<UplHistoryEvent>;
  /// Set UPL history event
  fn set_history(&mut self, event: UplHistoryEvent) -> &Upl;
  /// Get UPL object creation time
  fn get_created_at(&self) -> DateTime<Utc>;
  /// Get UPL object created by value (user id)
  fn get_created_by(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UplHistoryItem {
  event: UplHistoryEvent,
  created_at: DateTime<Utc>,
  created_by: String,
}

impl Default for UplHistoryItem {
  fn default() -> Self {
    Self {
      event: UplHistoryEvent::default(),
      created_at: Utc::now(),
      created_by: "".to_string(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UplHistoryEvent {
  // When UPL is created
  Created {
    by: String,
  },
  // When UPL is archived
  Archived,
  // When UPL is moved to a new location
  Moved {
    from: Location,
    to: Location,
  },
  // When best_before updated
  BestBeforeUpdated {
    to: NaiveDate,
  },
  // When UPL is locked
  Locked {
    to: Lock,
  },
  // When UPL is unlocked
  Unlocked,
  // When UPL is set as deprecated
  SetDeprecated {
    id: Option<i32>,
    comment: Option<String>,
  },
  // When UPL has set a special deprecation retail price
  SetDeprecatedPrice {
    retail_net_price: Option<u32>,
  },
  // Default event
  None,
}

impl Default for UplHistoryEvent {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Location {
  // Upl is in stock
  Stock(u32),
  // Upl is in a delivery
  Delivery(u32),
  // Upl is in a cart (closed purchase)
  Cart(u32),
  // Upl is missing and was moved to be discarded
  // but if its re-found
  Discard(u32),
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
    // Derived UPLs
    successors: Vec<u32>,
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
/// None means there is no lock, so the UPL can be moved away.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Lock {
  // Apply Price lock when new UPLs arrive from
  // Procurement and their prices need to be check
  // Pricing service must check this UPLs (PriceLock, SKU)
  // and create a re-pricing task or doesn't. When the new UPLs
  // pricing are okay, or the pricing task is done, pricing module
  // can unlock this procurement lock.
  Price(u32),
  // Cart lock means the given UPL is locked to a specific Cart
  // so it cannot move away, as its under a sales process.
  // Using when a UPL is in a Cart
  Cart(u32),
  // Apply Delivery Lock when the UPL is going to
  // be selected to a delivery between stocks.
  Delivery(u32),
  // Using when UPL is under an inventory process
  // and/or missing. We can set an Inventory lock to UPLs
  // that cannot be the part of the sales process due to
  // inventory/quality issues, and further decision is needed.
  // Or we can use inventory lock as a general lock for UPLs
  // that are a part of an inventory check process, and we don't want
  // the sales process to disturb that check process. And we don't want
  // the inventory process to cause delay in sales process. This means
  // inventory process must be very quick.
  Inventory(u32),
  // UPL has no lock
  // it can be updated and moved freely
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
  pub id: u32,
  // UPL Kind
  // Single or Bulk(u32)
  // Single means its a single UPL,
  // Bulk means its a collection of UPLs under a single UPL ID
  // e.g. a pallet flower soil (50)
  pub kind: Kind,
  // * Procurement
  pub procurement_id: u32,
  // Net wholesale price in which
  // this item was purchased by us
  pub procurement_net_price: u32,
  // Current UPL location
  pub location: Location, // todo? this way?
  // todo! Not NOW!
  // todo! Implement => location_history: Vec<Location>,
  // --
  // If the product is injured
  // it should be scraped. This field
  // contains the related scrap id
  pub deprecation_id: Option<i32>,
  // Related scrap comment
  // if there any
  // From the sku scrap comment from the
  // related scrap record
  pub deprecation_comment: Option<String>,
  // Related scrap price
  // if there any.
  // Can set if there is related scrap_id
  pub depreciation_retail_net_price: Option<u32>,
  // Best before date
  // Only for perishable goods.
  // Optional, but when we have one, we use
  // NaiveDate
  pub best_before: Option<NaiveDate>,
  // Product quantity
  // It contains Simple or Complex quantity
  // Or when a Simple product - which is divisible -
  // is divided, it contains the remained quantity.
  // Inherited from Product(service), but after
  // become Partial(u32), it's going to be managed
  // here without responding the related Product changes.
  // --
  // Only some, if Sku can be divided, and its unopened.
  // Once its opened, this amount will be none, and its
  // value is moved to its kind component
  // This value represents the SKU original divisible quantity
  pub divisible_amount: Option<u32>,
  // Lock enum
  // When a UPL is locked by any reason,
  // that UPL cannot be updated.
  // ~ Only ~ the lock owner can unlock
  pub lock: Lock,
  // UPL event history
  // We store all the major UPL event here
  pub history: Vec<UplHistoryItem>,
}

impl Upl {
  pub fn get_product_id(&self) -> u32 {
    match self.kind {
      Kind::Sku { product_id, sku: _ } => product_id,
      Kind::BulkSku { product_id, sku: _ } => product_id,
      Kind::OpenedSku {
        product_id,
        sku: _,
        amount: _,
      } => product_id,
      Kind::DerivedProduct {
        product_id,
        derived_from: _,
        amount: _,
      } => product_id,
    }
  }
  pub fn get_sku(&self) -> Option<u32> {
    match self.kind {
      Kind::Sku { product_id: _, sku } => Some(sku),
      Kind::BulkSku { product_id: _, sku } => Some(sku),
      Kind::OpenedSku {
        product_id: _,
        sku,
        amount: _,
      } => Some(sku),
      Kind::DerivedProduct {
        product_id: _,
        derived_from: _,
        amount: _,
      } => None,
    }
  }
}

impl Default for Upl {
  fn default() -> Self {
    Self {
      id: 0,
      kind: Kind::default(),
      procurement_id: 0,
      procurement_net_price: 0,
      location: Location::default(),
      deprecation_id: None,
      deprecation_comment: None,
      depreciation_retail_net_price: None,
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
