use std::ops::Mul;

use chrono::prelude::*;
use gzlib::id::LuhnCheck;
use packman::VecPackMember;
use serde::{Deserialize, Serialize};

/// UPL method declarations
pub trait UplMethods
where
  Self: Sized,
{
  /// Create a new UPL Object
  /// by the given details
  /// Should be used only by the procurement service
  /// ID Should be validated
  fn new(
    upl_id: String,
    product_id: u32,
    product_unit: String,
    sku: u32,
    piece: u32,
    sku_divisible_amount: u32,
    sku_divisible: bool,
    sku_net_price: u32,
    sku_vat: VAT,
    procurement_id: u32,
    sku_procurement_net_price: u32,
    location: Location,
    best_before: Option<DateTime<Utc>>,
    is_opened: bool,
    created_by: u32,
  ) -> Result<Self, String>;
  /// Get UPL ID ref
  fn get_upl_id(&self) -> &str;
  /// Get related product ID
  fn get_product_id(&self) -> u32;
  /// Get related SKU ID
  fn get_sku(&self) -> u32;
  /// Get UPL Kind ref
  fn get_kind(&self) -> &Kind;
  /// Get UPL procurement ID ref
  fn get_procurement_id(&self) -> u32;
  /// Get UPL procurement net price ref
  fn get_procurement_net_price(&self) -> u32;
  /// Check whether UPL can move to a different location
  /// depends on its acquired Lock kind
  fn can_move(&self, to: &Location) -> bool;
  /// Returns true IF has NO LOCK
  fn is_available(&self) -> bool;
  /// Returns true if a UPL has original package
  /// un-opened and healthy:
  ///  - no depreciation
  ///  - no best_before issue
  fn is_available_healthy(&self) -> bool;
  /// Get current location ref
  fn get_location(&self) -> &Location;
  /// Try move UPL to location B
  fn move_upl(&mut self, to: Location, created_by: u32) -> Result<&Self, String>;
  /// Check whether UPL has a lock or none
  fn has_lock(&self) -> bool;
  /// Get UPL lock ref
  fn get_lock(&self) -> &Lock;
  /// Check whether it can be locked to a &Lock
  fn can_lock(&self) -> bool;
  /// Try to lock UPL by a given Lock
  fn lock(&mut self, lock: Lock, created_by: u32) -> Result<&Self, String>;
  /// Try to unlock UPL
  fn unlock(&mut self, lock: Lock, created_by: u32) -> Result<&Self, String>;
  /// Unlock UPL anyway
  fn unlock_forced(&mut self) -> &Self;
  /// Try to set new price to UPL
  fn set_price(&mut self, sku_net_price: u32, sku_vat: VAT) -> Result<&Self, String>;
  /// Set depreciation
  /// Should be limited to the inventory service
  fn set_depreciation(
    &mut self,
    deprecation_id: u32,
    comment: String,
    created_by: u32,
  ) -> Result<&Self, String>;
  /// Remove deprecation
  fn remove_deprecation(&mut self, created_by: u32) -> Result<&Self, String>;
  /// Set depreciation price
  /// there is room for validation if needed
  fn set_depreciation_price(
    &mut self,
    net_depreciated_price: Option<u32>,
    created_by: u32,
  ) -> Result<&Self, String>;
  /// Check if the UPL is depreciated
  /// This can mean a damaged package, or anything the might
  /// lower the UPL value, but it can still be sold.
  fn is_depreciated(&self) -> bool;
  /// Get depreciation ID if there is any
  fn get_depreciation_id(&self) -> Option<u32>;
  /// Get depreciation comment if there is any
  fn get_depreciation_comment(&self) -> Option<&String>;
  /// Get depreciation price if there is any
  fn get_depreciation_price(&self) -> Option<u32>;
  /// Get best before date if there is any
  fn get_best_before(&self) -> Option<DateTime<Utc>>;
  /// Update UPL best_before date
  /// for any reason
  /// Should be private and used only from the inventory service
  fn set_best_before(&mut self, best_before: Option<DateTime<Utc>>, created_by: u32) -> &Self;
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
  /// ID must be validated
  fn split(&mut self, new_upl_id: String, piece: u32, created_by: u32) -> Result<Upl, String>;
  /// Split multiple UPLs from the bulk ones
  /// ----------
  /// IMPORTANT!
  /// ----------
  /// in a higher lever you must save the split UPL in the UPL store
  /// IDs must be validated
  fn split_bulk(&mut self, new_upl_ids: Vec<String>, created_by: u32) -> Result<Vec<Upl>, String>;
  /// Divide a divisible UPL into two UPLs
  /// If the UPL is a divisible Sku, then it will become an OpenedSku
  /// and the resulted new Upl will be a DerivedProduct
  /// ----------
  /// IMPORTANT!
  /// ----------
  /// in a higher lever you must save the split UPL in the UPL store
  /// ID must be validated
  fn divide(
    &mut self,
    new_upl_id: String,
    requested_amount: u32,
    created_by: u32,
  ) -> Result<Upl, String>;
  /// Try to merge a source and a derived UPL into together
  /// When for any reason we want to put back a derived UPL into
  /// its ancestor
  fn merge(&mut self, upl_to_destroy: Upl, created_by: u32) -> Result<&Upl, String>;

  /// Check whether this UPL is divisible or not
  fn is_divisible(&self) -> bool;
  /// If the UPL is divisible,
  /// returns the remaining amount that can be divide
  fn get_divisible_amount(&self) -> Option<u32>;
  /// Get UPL history
  fn get_history(&self) -> &Vec<UplHistoryItem>;
  /// Set UPL history event
  fn set_history(&mut self, event: UplHistoryItem) -> &Self;
  /// Get UPL object creation time
  fn get_created_at(&self) -> DateTime<Utc>;
  /// Get UPL object created by value (user id)
  fn get_created_by(&self) -> u32;
  /// Get UPL net_price
  fn get_upl_net_price(&self) -> u32;
  /// Get UPL vat
  fn get_upl_gross_price(&self) -> u32;
  /// Get UPL gross price
  fn get_upl_vat(&self) -> VAT;
  /// Get UPL has special price
  fn get_upl_has_special_price(&self) -> bool;
  /// Get net special price if there is any
  fn get_upl_special_price_net(&self) -> Option<u32>;
  /// Get net special margin if there is any
  fn get_upl_special_price_margin(&self) -> Option<u32>;
  /// Recalculate retail prices, procurement value and net margin
  fn recalculate_prices(&mut self);
  /// Try to open Kind Sku
  fn open(&mut self) -> Result<&Upl, String>;
  /// Try to close Kind OpenedSku
  fn close(&mut self) -> Result<&Upl, String>;
  /// Set UPL to be divisible based on its SKU
  fn set_divisible(&mut self, divisible: bool) -> &Self;
  /// Set Product unit
  fn set_product_unit(&mut self, unit: String) -> &Self;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreatedBy {
  // When the action is made by a User
  Uid(u32),
  // When the action is made by the software
  Technical,
}

impl Default for CreatedBy {
  fn default() -> Self {
    Self::Technical
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UplHistoryItem {
  event: UplHistoryEvent,
  created_at: DateTime<Utc>,
  created_by: CreatedBy,
}

impl UplHistoryItem {
  fn new(created_by: CreatedBy, event: UplHistoryEvent) -> Self {
    Self {
      event,
      created_at: Utc::now(),
      created_by,
    }
  }
}

impl Default for UplHistoryItem {
  fn default() -> Self {
    Self {
      event: UplHistoryEvent::default(),
      created_at: Utc::now(),
      created_by: CreatedBy::default(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UplHistoryEvent {
  // When UPL is created
  Created,
  // When UPL is archived
  Archived,
  // When UPL is moved to a new location
  Moved {
    from: Location,
    to: Location,
  },
  // When best_before updated
  BestBeforeUpdated {
    to: Option<DateTime<Utc>>,
  },
  // When UPL is locked
  Locked {
    to: Lock,
  },
  // When UPL is unlocked
  Unlocked,
  // When UPL is set as deprecated
  SetDeprecated {
    depreciation_id: u32,
    comment: String,
  },
  // Deprecation removed
  DeprecationRemoved,
  // When UPL has set a special depreciation retail price
  SetDepreciatedPrice {
    retail_net_price: Option<u32>,
  },
  Split {
    new_upl_id: String,
  },
  // When a divisible UPL has divided into a smaller part
  Divided {
    new_upl_id: String,
    requested_amount: u32,
  },
  // Default event
  None,
}

impl Default for UplHistoryEvent {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Location {
  // Upl is in stock
  Stock(u32),
  // Upl is in a delivery
  Delivery(u32),
  // Upl is in a cart (closed purchase)
  Cart(String),
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
    sku: u32,
  },
  // Muliple un-opened SKU in a bulk package
  // sub UPLs cannot have a UPL ID yet, but we all of them
  // share the same UPL attributes. So when we split this bulk
  // package we create the UPLs by cloning its attributes.
  BulkSku {
    sku: u32,
    upl_pieces: u32,
  },
  // An opened sku
  // An original sku has expanded,
  // and a part of it is already out of it.
  OpenedSku {
    sku: u32,
    amount: u32,
    // Derived UPLs
    successors: Vec<String>,
  },
  // Piece of product that
  // derives from an opened sku
  DerivedProduct {
    // Derived from this UPL
    derived_from: String,
    // Derived from this SKU
    derived_from_sku: u32,
    // Amount in the products unit
    amount: u32,
  },
}

impl Default for Kind {
  fn default() -> Self {
    Self::Sku { sku: 0 }
  }
}

/// Lock kinds
/// None means there is no lock, so the UPL can be moved away.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Lock {
  // Cart lock means the given UPL is locked to a specific Cart
  // so it cannot move away, as its under a sales process.
  // Using when a UPL is in a Cart
  Cart(String),
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

impl Lock {
  // Behaves like Option<T>
  pub fn is_none(&self) -> bool {
    match self {
      Lock::None => true,
      _ => true,
    }
  }
  // Behaves like Option<T>
  // pub fn is_some(&self) -> bool {
  //   !self.is_none()
  // }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Depreciation {
  // If the product is injured
  // it should be depreciated. This field
  // contains the related depreciation id
  pub depreciation_id: u32,
  // Related scrap price
  // if there any.
  // Can set if there is related depreciation_id
  pub net_retail_price: Option<u32>,
  // UPL net margin when we have special price
  pub margin_net: Option<u32>,
  // Related depreciation comment
  // if there any
  // From the sku scrap comment from the
  // related depreciation record
  pub comment: String,
}

impl Depreciation {
  /// Create a new depreciation object
  pub fn new(depreciation_id: u32, comment: String) -> Self {
    Self {
      depreciation_id,
      net_retail_price: None,
      margin_net: None,
      comment,
    }
  }
  /// Set depreciation price
  pub fn set_price(&mut self, net_retail_price: Option<u32>, margin_net: Option<u32>) {
    self.net_retail_price = net_retail_price;
    self.margin_net = margin_net;
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum VAT {
  AAM,
  FAD,
  TAM,
  _5,
  _18,
  _27,
}

impl Default for VAT {
  fn default() -> Self {
    VAT::_27
  }
}

impl VAT {
  pub fn from_str(str: &str) -> Result<VAT, String> {
    match str {
      "AAM" => Ok(VAT::AAM),
      "aam" => Ok(VAT::AAM),
      "FAD" => Ok(VAT::FAD),
      "fad" => Ok(VAT::FAD),
      "TAM" => Ok(VAT::TAM),
      "tam" => Ok(VAT::TAM),
      "5" => Ok(VAT::_5),
      "18" => Ok(VAT::_18),
      "27" => Ok(VAT::_27),
      _ => Err("Nem megfelelő Áfa formátum! 5, 18, 27, AAM, TAM, FAD".into()),
    }
  }
}

impl ToString for VAT {
  fn to_string(&self) -> String {
    match self {
      VAT::AAM => "AAM".to_string(),
      VAT::FAD => "FAD".to_string(),
      VAT::TAM => "TAM".to_string(),
      VAT::_5 => "5".to_string(),
      VAT::_18 => "18".to_string(),
      VAT::_27 => "27".to_string(),
    }
  }
}

impl Mul<VAT> for u32 {
  type Output = u32;

  fn mul(self, rhs: VAT) -> Self::Output {
    let res = match rhs {
      VAT::AAM => self as f32 * 1.0,
      VAT::FAD => self as f32 * 1.0,
      VAT::TAM => self as f32 * 1.0,
      VAT::_5 => self as f32 * 1.05,
      VAT::_18 => self as f32 * 1.18,
      VAT::_27 => self as f32 * 1.27,
    };
    res.round() as u32
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Upl {
  // Unique UPL ID
  // String
  pub id: String,
  // Related product ID
  pub product_id: u32,
  // 1 if its not divisible
  pub sku_divisible_amount: u32,
  // Related Product unit
  pub product_unit: String,
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
  // SKU original procurement price
  pub procurement_net_price_sku: u32,
  // Total net margin for this UPL
  pub margin_net: u32,
  // Current UPL location
  pub location: Location,
  // todo! Not NOW!
  // todo! Implement => location_history: Vec<Location>,
  // Depreciation
  pub depreciation: Option<Depreciation>,
  // Best before date
  // Only for perishable goods.
  // Optional, but when we have one, we use
  // DateTime<Utc>
  pub best_before: Option<DateTime<Utc>>,
  // SKU is divisible or not
  pub sku_divisible: bool,
  // Stored sku net price
  pub sku_price_net: u32,
  // Net retail price
  pub price_net: u32,
  // SKU VAT
  pub vat: VAT,
  // Gross retail price
  pub price_gross: u32,
  // Lock enum
  // When a UPL is locked by any reason,
  // that UPL cannot be updated.
  // ~ Only ~ the lock owner can unlock
  pub lock: Lock,
  // UPL event history
  // We store all the major UPL event here
  pub history: Vec<UplHistoryItem>,
  // UPL object creation time
  pub created_at: DateTime<Utc>,
  // UPL object created by (user id)
  pub created_by: u32,
}

impl UplMethods for Upl {
  fn new(
    upl_id: String,
    product_id: u32,
    product_unit: String,
    sku: u32,
    piece: u32,
    sku_divisible_amount: u32,
    sku_divisible: bool,
    sku_price_net: u32,
    sku_vat: VAT,
    procurement_id: u32,
    procurement_net_price_sku: u32,
    location: Location,
    best_before: Option<DateTime<Utc>>,
    is_opened: bool,
    created_by: u32,
  ) -> Result<Self, String> {
    // Create new UPL
    let mut upl = Self {
      // Check if ID is Luhn valid
      id: upl_id
        .luhn_check()
        .map_err(|_| "A megadott UPL ID nem valid!".to_string())?,
      product_id,
      kind: match is_opened {
        true => Kind::OpenedSku {
          sku: sku,
          amount: piece,
          successors: Vec::new(),
        },
        false => match piece {
          x if x > 1 => Kind::BulkSku {
            sku: sku,
            upl_pieces: x,
          },
          _ => Kind::Sku { sku: sku },
        },
      },
      product_unit,
      procurement_id,
      procurement_net_price: 0,
      procurement_net_price_sku,
      location,
      depreciation: None,
      best_before,
      lock: Lock::None,
      // Init history vector with UplHistoryEvent::Created
      history: vec![UplHistoryItem::new(
        CreatedBy::Uid(created_by.clone()),
        UplHistoryEvent::Created,
      )],
      created_at: Utc::now(),
      created_by,
      sku_divisible_amount,
      margin_net: 0,
      sku_divisible,
      sku_price_net,
      price_net: 0,
      vat: sku_vat,
      price_gross: 0,
    };

    // Set prices
    upl.recalculate_prices();

    // Return new UPL
    Ok(upl)
  }

  fn get_product_id(&self) -> u32 {
    self.product_id
  }

  fn get_sku(&self) -> u32 {
    match self.kind {
      Kind::Sku { sku } => sku,
      Kind::BulkSku { sku, upl_pieces: _ } => sku,
      Kind::OpenedSku {
        sku,
        amount: _,
        successors: _,
      } => sku,
      Kind::DerivedProduct {
        derived_from: _,
        derived_from_sku,
        amount: _,
      } => derived_from_sku,
    }
  }

  fn get_kind(&self) -> &Kind {
    &self.kind
  }

  fn get_procurement_id(&self) -> u32 {
    self.procurement_id
  }

  fn get_procurement_net_price(&self) -> u32 {
    self.procurement_net_price
  }

  fn can_move(&self, to: &Location) -> bool {
    match to {
      Location::Stock(_) => match self.lock {
        Lock::Cart(_) => false,
        Lock::Delivery(_) => false,
        Lock::Inventory(_) => false,
        // Or if it has no lock at all
        Lock::None => true,
      },
      Location::Delivery(id) => match self.lock {
        Lock::Cart(_) => false,
        // If it has already a delivery lock,
        // then only if it has its own delivery lock
        Lock::Delivery(_id) => *id == _id,
        Lock::Inventory(_) => false,
        // Or if it has no lock at all
        Lock::None => true,
      },
      Location::Cart(id) => match &self.lock {
        // Only if it has its own cart lock
        Lock::Cart(_id) => id == _id,
        Lock::Delivery(_) => false,
        Lock::Inventory(_) => false,
        // Or if it has no lock at all
        Lock::None => true,
      },
      Location::Discard(_) => match self.lock {
        Lock::Cart(_) => false,
        Lock::Delivery(_) => false,
        // Only inventory locked UPL can be moved to Discard
        Lock::Inventory(_) => true,
        Lock::None => false,
      },
    }
  }

  fn get_location(&self) -> &Location {
    &self.location
  }

  fn move_upl(&mut self, to: Location, created_by: u32) -> Result<&Self, String> {
    // Check whether it can move to the target location or not
    if !self.can_move(&to) {
      return Err("Cannot move to target location".into());
    }
    // Preserve from_location to save later into history
    let from = self.location.clone();
    // If it can move
    // then move it to there
    self.location = to.clone();
    // Set history event
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::Moved { from, to },
    ));
    // Should clear lock after move
    self.unlock_forced();
    Ok(self)
  }

  fn has_lock(&self) -> bool {
    match self.lock {
      Lock::None => false,
      _ => true,
    }
  }

  fn get_lock(&self) -> &Lock {
    &self.lock
  }

  fn can_lock(&self) -> bool {
    // Can lock only if there is no lock applied
    // otherwise you need to unlock it first
    // then try to apply the new lock
    self.get_lock().is_none()
  }

  fn lock(&mut self, lock: Lock, created_by: u32) -> Result<&Self, String> {
    // Check if wheter we can lock it or not
    if !self.can_lock() {
      return Err("Cannot lock! Already locked!".into());
    }
    // Set the new lock
    self.lock = lock.clone();
    // Set lock history event
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::Locked { to: lock },
    ));
    // Return &self
    Ok(self)
  }

  fn unlock(&mut self, lock: Lock, created_by: u32) -> Result<&Self, String> {
    match self.lock == lock {
      true => {
        // Just release the lock
        self.lock = Lock::None;
        // Set UPL history
        self.set_history(UplHistoryItem::new(
          CreatedBy::Uid(created_by),
          UplHistoryEvent::Unlocked,
        ));
        // Return self ref
        Ok(self)
      }
      false => Err("A kért UPL zárolása nem fololdható. Nem megfelelő a forrás zárlat!".into()),
    }
  }

  fn unlock_forced(&mut self) -> &Self {
    // Just release the lock
    self.lock = Lock::None;
    // Set UPL history
    self.set_history(UplHistoryItem::new(
      CreatedBy::Technical,
      UplHistoryEvent::Unlocked,
    ));
    // Return self ref
    self
  }

  fn set_depreciation(
    &mut self,
    depreciation_id: u32,
    comment: String,
    created_by: u32,
  ) -> Result<&Self, String> {
    // Check whether already depreciated
    if self.depreciation.is_some() {
      return Err("A termék már selejtezett!".into());
    }

    // Set depreciation
    self.depreciation = Some(Depreciation::new(depreciation_id, comment.clone()));

    // Set UPL history
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::SetDeprecated {
        depreciation_id,
        comment,
      },
    ));
    // Return Ok self ref
    Ok(self)
  }

  fn remove_deprecation(&mut self, created_by: u32) -> Result<&Self, String> {
    if self.depreciation.is_none() {
      return Err("A UPL nem selejtezett!".to_string());
    }
    self.depreciation = None;
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::DeprecationRemoved,
    ));
    Ok(self)
  }

  fn set_depreciation_price(
    &mut self,
    net_retail_price: Option<u32>,
    created_by: u32,
  ) -> Result<&Self, String> {
    // Set depreciation price if there is deprecation already set
    if let Some(dep) = &mut self.depreciation {
      // Calculate margin for the given depreciation price
      let margin: Option<u32> = match net_retail_price {
        Some(discounted_price) => Some(discounted_price - self.procurement_net_price),
        None => None,
      };
      // Set depreciation price
      dep.set_price(net_retail_price, margin);
    } else {
      return Err("UPL is not depreciated!".to_string());
    }

    // Set UPL history
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::SetDepreciatedPrice {
        retail_net_price: net_retail_price,
      },
    ));

    // Return Self as ref
    Ok(self)
  }

  fn is_depreciated(&self) -> bool {
    self.depreciation.is_some()
  }

  fn get_depreciation_id(&self) -> Option<u32> {
    if let Some(dep) = &self.depreciation {
      return Some(dep.depreciation_id);
    }
    None
  }

  fn get_depreciation_comment(&self) -> Option<&String> {
    if let Some(dep) = &self.depreciation {
      return Some(&dep.comment);
    }
    None
  }

  fn get_depreciation_price(&self) -> Option<u32> {
    if let Some(dep) = &self.depreciation {
      return dep.net_retail_price;
    }
    None
  }

  fn get_best_before(&self) -> Option<DateTime<Utc>> {
    self.best_before
  }

  fn set_best_before(&mut self, best_before: Option<DateTime<Utc>>, created_by: u32) -> &Self {
    // Update best_before date
    self.best_before = best_before;
    // Update UPL history
    self.set_history(UplHistoryItem::new(
      CreatedBy::Uid(created_by),
      UplHistoryEvent::BestBeforeUpdated { to: best_before },
    ));
    // Return Self as ref
    self
  }

  fn is_original(&self) -> bool {
    match self.kind {
      Kind::Sku { sku: _ }
      | Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => true,
      _ => false,
    }
  }

  fn is_bulk(&self) -> bool {
    match self.kind {
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => true,
      _ => false,
    }
  }

  fn get_upl_piece(&self) -> u32 {
    match self.kind {
      Kind::BulkSku { sku: _, upl_pieces } => upl_pieces,
      _ => 1,
    }
  }

  fn split(&mut self, new_upl_id: String, piece: u32, created_by: u32) -> Result<Upl, String> {
    // Check piece
    if piece == 0 {
      return Err("Az új UPL mennyiség nem lehet 0!".to_string());
    }

    if self.has_lock() {
      return Err("A termékből nem tudunk leválasztani, mivel zárolva van!".to_string());
    }

    // Check if new upl id is valid Luhn
    new_upl_id
      .luhn_check_ref()
      .map_err(|_| "Az új UPL ID invalid!".to_string())?;

    match self.kind {
      Kind::BulkSku {
        sku,
        ref mut upl_pieces,
      } => {
        match upl_pieces {
          // Check if we have more then the requested piece in bulk
          &mut x if x > piece => {
            // Decrease UPL bulk pieces by one
            *upl_pieces -= piece;
            // Clone itself as a new UPL
            let mut new_upl = self.clone();
            // Update its kind to be a single Sku UPL
            // and copy the product and sku ids
            new_upl.kind = match piece {
              x if x == 0 => return Err("Az új UPL mennyiség nem lehet 0!".to_string()),
              x if x == 1 => Kind::Sku { sku: sku },
              _ => Kind::BulkSku {
                sku: sku,
                upl_pieces: piece,
              },
            };
            // Set UPL history
            self.set_history(UplHistoryItem::new(
              CreatedBy::Uid(created_by),
              UplHistoryEvent::Split { new_upl_id },
            ));

            // Recalculate parent prices
            self.recalculate_prices();

            // Recalculate child prices
            new_upl.recalculate_prices();

            // Return the new UPL
            Ok(new_upl)
          }
          _ => Err("A gyüjtő mérete nem nagyobb, mint a kért mennyiség!".to_string()),
        }
      }
      _ => Err("Az adott UPL-t nem lehet szét választani, nem tömeges UPL!".into()),
    }
  }

  fn split_bulk(&mut self, new_upl_ids: Vec<String>, created_by: u32) -> Result<Vec<Upl>, String> {
    // Check all the new UPL IDs to ensure all of them valid Luhn ids
    for id in &new_upl_ids {
      if id.luhn_check_ref().is_err() {
        return Err(format!("Az alábbi új UPL ID invalid! {}", id));
      }
    }

    if self.has_lock() {
      return Err("A termékből nem tudunk szétválasztani, mivel zárolva van!".to_string());
    }

    match self.kind {
      Kind::BulkSku { sku: _, upl_pieces } => {
        if upl_pieces as usize <= new_upl_ids.len() {
          return Err("A UPL nem elég nagy, hogy a kért mennyiséget leválasszuk róla!".to_string());
        }
        let mut result = Vec::new();
        new_upl_ids.into_iter().for_each(|id| {
          // We can use unwrap, as we already checked its a Bulk UPL
          // and only this can cause error
          if let Ok(upl) = self.split(id, 1, created_by.clone()) {
            result.push(upl);
          }
        });
        Ok(result)
      }
      _ => Err("A kért UPL nem szétválasztható!".to_string()),
    }
  }

  fn open(&mut self) -> Result<&Upl, String> {
    if !self.is_divisible() {
      return Err(
        "A kért UPL nem mérhető ki, így nem bontható meg!
        Vagy nem kimérhető a SKU, vagy a kimérhető mennyiség 1, ami nem elgendő."
          .to_string(),
      );
    }

    if self.has_lock() {
      return Err("A terméket nem tudjuk megnyitni, mivel zárolva van!".to_string());
    }

    match &self.get_divisible_amount() {
      Some(_) => (),
      None => return Err("A megadott SKU nem mérhető ki!".into()),
    };

    match &mut self.kind {
      Kind::Sku { sku } => {
        // We change the UPL kind to be OpenedSku
        // and fill it with the previous data
        self.kind = Kind::OpenedSku {
          sku: *sku,
          amount: self.sku_divisible_amount,
          successors: Vec::new(),
        };

        // Return self ref
        Ok(self)
      }
      _ => Err("A kért terméket nem lehet megbontani, mert vagy gyüjtő, vagy már bontott.".into()),
    }
  }

  fn close(&mut self) -> Result<&Upl, String> {
    if self.has_lock() {
      return Err("A terméket nem tudjuk lezárni, mivel zárolva van!".to_string());
    }

    match &mut self.kind {
      Kind::OpenedSku {
        sku,
        amount,
        successors: _,
      } => {
        // Check if its original
        if *amount != self.sku_divisible_amount {
          return Err("A termékből már kimértek, így nem zárható vissza.".to_string());
        }
        // Set Kind::Sku again
        self.kind = Kind::Sku { sku: *sku };

        // Return self ref
        Ok(self)
      }
      _ => Err("A kért nem bontott termék, így nem lehet vissza zárni.".into()),
    }
  }

  fn divide(
    &mut self,
    new_upl_id: String,
    requested_amount: u32,
    created_by: u32,
  ) -> Result<Upl, String> {
    // Check piece
    if requested_amount == 0 {
      return Err("Nem lehet 0 egységet kimérni!".to_string());
    }

    if self.has_lock() {
      return Err("A termékből nem tudunk kimérni, mivel zárolva van!".to_string());
    }

    // Check new_upl_id is valid Luhn
    new_upl_id
      .luhn_check_ref()
      .map_err(|_| "Az új UPL id invalid!".to_string())?;

    match &mut self.kind {
      Kind::OpenedSku {
        sku: _,
        ref mut amount,
        ref mut successors,
      } => {
        // Check if there is enough amount inside this UPL
        if *amount <= requested_amount {
          return Err("A kért termék túl kicsi a kívánt mértékhez!".into());
        }

        // Decrease its amount
        *amount -= requested_amount;

        // Set its new successor
        successors.push(new_upl_id.clone());

        // Clone itself
        let mut new_upl = self.clone();

        // Set new ID
        new_upl.id = new_upl_id.clone();

        // Set created by
        new_upl.created_by = created_by.clone();

        // Set created at
        new_upl.created_at = Utc::now();

        // Set the new UPLs kind to be a derived product
        new_upl.kind = Kind::DerivedProduct {
          derived_from: self.id.clone(),
          derived_from_sku: self.get_sku(),
          amount: requested_amount,
        };

        // Recalculate parent prices
        self.recalculate_prices();

        // Recalculate child prices
        new_upl.recalculate_prices();

        // Return the new UPL
        Ok(new_upl)
      }
      // We cannot divide a derived UPL
      _ => Err("A kért termék nem mérhető ki! Csak bontott termék mérhető ki!".into()),
    }
  }

  fn merge(&mut self, upl_to_merge: Upl, _by: u32) -> Result<&Upl, String> {
    if self.is_depreciated() {
      return Err(
        "A szülő UPL selejtezett. Selejtezett termékbe nem tudunk vissza tenni".to_string(),
      );
    }

    if self.has_lock() {
      return Err("Nem tehetjük vissza a terméket, mert a szülő termék zárolva van!".to_string());
    }

    if upl_to_merge.has_lock() {
      return Err("Nem tehetjük vissza a terméket, mert az zárolva van!".to_string());
    }
    // Try merge back
    // and calculate new amount, prices, procurement value and margin
    // Don't forget to remove the merged UPL from Upl DB
    match &mut self.kind {
      Kind::OpenedSku {
        sku: _,
        amount: ref mut amount_parent,
        successors: _,
      } => match &upl_to_merge.kind {
        Kind::DerivedProduct {
          derived_from,
          derived_from_sku: _,
          amount: child_amount,
        } => {
          if &self.id != derived_from {
            return Err("A kért UPL nem tehető vissza másik szülőbe!".to_string());
          }
          // Put back the required amount
          *amount_parent = *amount_parent + *child_amount;
          // Recalculate prices, margin + procurement net value
          self.recalculate_prices();
          // Return self as ref
          return Ok(self);
        }
        _ => return Err("A kért UPL nem kimért UPL, nem tehető vissza!".to_string()),
      },
      _ => return Err("A cél UPL nem nyitott termék!".to_string()),
    }
  }

  fn is_divisible(&self) -> bool {
    match &self.kind {
      // Only true if sku_divisible AND divisible amount > 1
      Kind::Sku { sku: _ } => (self.sku_divisible_amount > 1) && self.sku_divisible,
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => false,
      Kind::OpenedSku {
        sku: _,
        amount,
        successors: _,
      } => *amount > 1,
      Kind::DerivedProduct {
        derived_from: _,
        derived_from_sku: _,
        amount: _,
      } => false,
    }
  }

  fn get_divisible_amount(&self) -> Option<u32> {
    match &self.kind {
      Kind::Sku { sku: _ } => Some(self.sku_divisible_amount),
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => None,
      Kind::OpenedSku {
        sku: _,
        amount,
        successors: _,
      } => Some(*amount),
      Kind::DerivedProduct {
        derived_from: _,
        derived_from_sku: _,
        amount: _,
      } => None,
    }
  }

  fn get_history(&self) -> &Vec<UplHistoryItem> {
    &self.history
  }

  fn set_history(&mut self, event: UplHistoryItem) -> &Self {
    self.history.push(event);
    self
  }

  fn get_created_at(&self) -> DateTime<Utc> {
    self.created_at
  }

  fn get_created_by(&self) -> u32 {
    self.created_by
  }

  fn is_available(&self) -> bool {
    // This will tell if this UPL is available for
    // purchase - even with strong considerations.
    //            --------------------------------
    // This returns even if the UPL is broken, damaged,
    // expired best_before.
    //
    // 1. Check has no lock
    //    Otherwise we return true in all cases
    !self.has_lock()
  }

  fn is_available_healthy(&self) -> bool {
    // 1. Check has no lock
    !self.has_lock()
    // 2. Check if it's not depreciated
    && self.get_depreciation_id().is_none()
    // 3. Check Kind::Sku || Kind::BulkSku
    && (match self.kind {
      Kind::Sku {
        sku: _,
      } => true,
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => true,
      _ => false,
    })
    // 4. Check has best before and if its still valid
    && (match self.best_before {
      Some(best_before) => best_before.date() >= Utc::now().date(),
      None => true,
    })
  }

  fn get_upl_id(&self) -> &str {
    &self.id
  }

  fn set_price(&mut self, sku_net_price: u32, sku_vat: VAT) -> Result<&Self, String> {
    // Store SKU net price
    self.sku_price_net = sku_net_price;
    // Store new VAT
    self.vat = sku_vat;
    // Recalculate prices
    self.recalculate_prices();
    // Return self as ref
    Ok(self)
  }

  fn get_upl_net_price(&self) -> u32 {
    match &self.depreciation {
      Some(d) => match d.net_retail_price {
        Some(dp) => dp,
        None => self.price_net,
      },
      None => self.price_net,
    }
  }

  fn get_upl_gross_price(&self) -> u32 {
    match &self.depreciation {
      Some(d) => match d.net_retail_price {
        Some(dp) => dp * self.vat,
        None => self.price_gross,
      },
      None => self.price_gross,
    }
  }

  fn get_upl_vat(&self) -> VAT {
    self.vat
  }

  fn get_upl_has_special_price(&self) -> bool {
    match &self.depreciation {
      Some(d) => d.net_retail_price.is_some(),
      None => false,
    }
  }

  fn recalculate_prices(&mut self) {
    match self.kind {
      // Set price for a normal SKU UPL
      Kind::Sku { sku: _ } => {
        // Set net retail price
        self.price_net = self.sku_price_net;
        // Set gross retail price
        self.price_gross = self.sku_price_net * self.vat;
      }
      // Set price for a normal BulkSku UPL
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => {
        // Set net retail price
        self.price_net = self.sku_price_net;
        // Set gross retail price
        self.price_gross = self.sku_price_net * self.vat;
      }
      // Set price for an opened SKU
      Kind::OpenedSku {
        sku: _,
        amount,
        successors: _,
      } => {
        // Calculate unit net price
        let unit_net_price = self.sku_price_net as f32 / self.sku_divisible_amount as f32;
        // Reset UPL retail net price based on its amount
        self.price_net = (amount as f32 * unit_net_price).round() as u32;
        // Reset UPL retail gross price based on its amount
        self.price_gross = self.price_net * self.vat;
        // Calculate unit procurement value
        let unit_procurement_value =
          self.procurement_net_price_sku as f32 / self.sku_divisible_amount as f32;
        // Set new procurement value
        self.procurement_net_price = (amount as f32 * unit_procurement_value).round() as u32;
      }
      Kind::DerivedProduct {
        derived_from: _,
        derived_from_sku: _,
        amount,
      } => {
        // Calculate unit net price
        let unit_net_price = self.sku_price_net as f32 / self.sku_divisible_amount as f32;
        // Reset UPL retail net price based on its amount
        self.price_net = (amount as f32 * unit_net_price).round() as u32;
        // Reset UPL retail gross price based on its amount
        self.price_gross = self.price_net * self.vat;
        // Calculate unit procurement value
        let unit_procurement_value =
          self.procurement_net_price_sku as f32 / self.sku_divisible_amount as f32;
        // Set new procurement value
        self.procurement_net_price = (amount as f32 * unit_procurement_value).round() as u32;
      }
    }
    // Set margin
    self.margin_net = self.price_net - self.procurement_net_price;
  }

  fn set_divisible(&mut self, divisible: bool) -> &Self {
    self.sku_divisible = divisible;
    self
  }

  fn set_product_unit(&mut self, unit: String) -> &Self {
    self.product_unit = unit;
    self
  }

  fn get_upl_special_price_net(&self) -> Option<u32> {
    match &self.depreciation {
      Some(d) => d.net_retail_price,
      None => None,
    }
  }

  fn get_upl_special_price_margin(&self) -> Option<u32> {
    match &self.depreciation {
      Some(d) => d.margin_net,
      None => None,
    }
  }
}

impl Default for Upl {
  fn default() -> Self {
    Self {
      id: "".to_string(),
      product_id: 0,
      product_unit: String::default(),
      kind: Kind::default(),
      procurement_id: 0,
      procurement_net_price: 0,
      location: Location::default(),
      depreciation: None,
      best_before: None,
      sku_divisible_amount: 1,
      lock: Lock::default(),
      history: Vec::new(),
      created_at: Utc::now(),
      created_by: 0,
      procurement_net_price_sku: 0,
      margin_net: 0,
      price_net: 0,
      vat: VAT::default(),
      price_gross: 0,
      sku_divisible: false,
      sku_price_net: 0,
    }
  }
}

impl VecPackMember for Upl {
  type Out = String;

  fn get_id(&self) -> &Self::Out {
    &self.id
  }
}
