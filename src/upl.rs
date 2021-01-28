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
    sku: u32,
    piece: u32,
    procurement_id: u32,
    procurement_net_price: u32,
    location: Location,
    best_before: Option<DateTime<Utc>>,
    divisible_amount: Option<u32>,
    is_opened: bool,
    created_by: u32,
  ) -> Result<Self, String>;
  /// Get UPL ID ref
  fn get_upl_id(&self) -> &str;
  /// Get related product ID
  fn get_product_id(&self) -> u32;
  /// Get related SKU ID
  fn get_sku(&self) -> Option<u32>;
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
  /// Set divisible amount
  fn set_divisible_amount(&mut self, divisible_amount: Option<u32>) -> Result<&Self, String>;
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
  fn split(&mut self, new_upl_id: String, created_by: u32) -> Result<Upl, String>;
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
  // fn merge(&mut self, upl_to_destroy: Upl, created_by: u32) -> Result<&Upl, String>;

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
  // Related depreciation comment
  // if there any
  // From the sku scrap comment from the
  // related depreciation record
  pub comment: String,
}

impl Depreciation {
  /// Create a new depreciation object
  pub fn new(depreciation_id: u32, net_retail_price: Option<u32>, comment: String) -> Self {
    Self {
      depreciation_id,
      net_retail_price,
      comment,
    }
  }
  /// Set depreciation price
  pub fn set_price(&mut self, net_retail_price: Option<u32>) {
    self.net_retail_price = net_retail_price;
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Upl {
  // Unique UPL ID
  // String
  pub id: String,
  // Related product ID
  pub product_id: u32,
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
  // Depreciation
  pub depreciation: Option<Depreciation>,
  // Best before date
  // Only for perishable goods.
  // Optional, but when we have one, we use
  // DateTime<Utc>
  pub best_before: Option<DateTime<Utc>>,
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
  // UPL object creation time
  pub created_at: DateTime<Utc>,
  // UPL object created by (user id)
  pub created_by: u32,
}

impl UplMethods for Upl {
  fn new(
    upl_id: String,
    product_id: u32,
    sku: u32,
    piece: u32,
    procurement_id: u32,
    procurement_net_price: u32,
    location: Location,
    best_before: Option<DateTime<Utc>>,
    divisible_amount: Option<u32>,
    is_opened: bool,
    created_by: u32,
  ) -> Result<Self, String> {
    // Or just do the validation in higher level?
    // Just do the validation and the duplicate check
    Ok(Self {
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
      procurement_id,
      procurement_net_price,
      location,
      depreciation: None,
      best_before,
      divisible_amount,
      lock: Lock::None,
      // Init history vector with UplHistoryEvent::Created
      history: vec![UplHistoryItem::new(
        CreatedBy::Uid(created_by.clone()),
        UplHistoryEvent::Created,
      )],
      created_at: Utc::now(),
      created_by,
    })
  }

  fn get_product_id(&self) -> u32 {
    self.product_id
  }

  fn get_sku(&self) -> Option<u32> {
    match self.kind {
      Kind::Sku { sku } => Some(sku),
      Kind::BulkSku { sku, upl_pieces: _ } => Some(sku),
      Kind::OpenedSku {
        sku,
        amount: _,
        successors: _,
      } => Some(sku),
      Kind::DerivedProduct {
        derived_from: _,
        amount: _,
      } => None,
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
    self.depreciation = Some(Depreciation::new(depreciation_id, None, comment.clone()));

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
      dep.set_price(net_retail_price);
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

  fn split(&mut self, new_upl_id: String, created_by: u32) -> Result<Upl, String> {
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
          // Check if we have more then 1 upls in bulk
          &mut x if x > 1 => {
            // Decrease UPL bulk pieces by one
            *upl_pieces -= 1;
            // Clone itself as a new UPL
            let mut new_upl = self.clone();
            // Update its kind to be a single Sku UPL
            // and copy the product and sku ids
            new_upl.kind = Kind::Sku { sku: sku };
            // Set UPL history
            self.set_history(UplHistoryItem::new(
              CreatedBy::Uid(created_by),
              UplHistoryEvent::Split { new_upl_id },
            ));
            // Return the new UPL
            Ok(new_upl)
          }
          _ => Err("A UPL csak 1 db-ot tartalmaz! Nem lehet tovább bontani!".to_string()),
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

    match self.kind {
      Kind::BulkSku { sku: _, upl_pieces } => {
        if upl_pieces as usize <= new_upl_ids.len() {
          return Err("A UPL nem elég nagy, hogy a kért mennyiséget leválasszuk róla!".to_string());
        }
        let mut result = Vec::new();
        new_upl_ids.into_iter().for_each(|id| {
          // We can use unwrap, as we already checked its a Bulk UPL
          // and only this can cause error
          if let Ok(upl) = self.split(id, created_by.clone()) {
            result.push(upl);
          }
        });
        Ok(result)
      }
      _ => Err("A kért UPL nem szétválasztható!".to_string()),
    }
  }

  fn divide(
    &mut self,
    new_upl_id: String,
    requested_amount: u32,
    created_by: u32,
  ) -> Result<Upl, String> {
    // Check new_upl_id is valid Luhn
    new_upl_id
      .luhn_check_ref()
      .map_err(|_| "Az új UPL id invalid!".to_string())?;

    match &mut self.kind {
      Kind::Sku { sku } => {
        let amount = match self.divisible_amount {
          Some(a) => a,
          None => return Err("A megadott SKU nem mérhető ki!".into()),
        };

        // Check if there is enough amount inside this UPL
        if amount <= requested_amount {
          return Err(
            "Túl nagy a kimérendő mennyiség! A termék kisebb, mint a kért mennyiség.".into(),
          );
        }

        // We change the UPL kind to be OpenedSku
        // and fill it with the previous data
        self.kind = Kind::OpenedSku {
          sku: *sku,
          amount: amount - requested_amount,
          successors: vec![new_upl_id.clone()],
        };

        // We reset the divisible amount field
        self.divisible_amount = None;

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
          amount: requested_amount,
        };

        // Set UPL history
        self.set_history(UplHistoryItem::new(
          CreatedBy::Uid(created_by),
          UplHistoryEvent::Divided {
            new_upl_id,
            requested_amount,
          },
        ));

        // Return the new UPL
        Ok(new_upl)
      }
      // We cannot divide a bulk UPL
      Kind::BulkSku {
        sku: _,
        upl_pieces: _,
      } => Err("A kért termék nem osztható! Előbb válassza szét őket!".into()),
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
          amount: requested_amount,
        };

        // Return the new UPL
        Ok(new_upl)
      }
      // We cannot divide a derived UPL
      Kind::DerivedProduct {
        derived_from: _,
        amount: _,
      } => Err("A kért termék egy kimért termék, amiből nem tudunk többet kimérni.".into()),
    }
  }

  // fn merge(&mut self, upl: Upl, by: String) -> Result<&Upl, String> {
  //   // TODO! Implement this!
  //   todo!()
  // }

  fn is_divisible(&self) -> bool {
    match &self.kind {
      Kind::Sku { sku: _ } => self.divisible_amount.is_some(),
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
        amount: _,
      } => false,
    }
  }

  fn get_divisible_amount(&self) -> Option<u32> {
    match &self.kind {
      Kind::Sku { sku: _ } => self.divisible_amount,
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

  fn set_divisible_amount(&mut self, divisible_amount: Option<u32>) -> Result<&Self, String> {
    match self.kind {
      Kind::Sku { sku: _ } => {
        self.divisible_amount = divisible_amount;
        Ok(self)
      }
      _ => Err("Csak egyedülálló, bontatlan UPL állítható be kimérésre".into()),
    }
  }
}

impl Default for Upl {
  fn default() -> Self {
    Self {
      id: "".to_string(),
      product_id: 0,
      kind: Kind::default(),
      procurement_id: 0,
      procurement_net_price: 0,
      location: Location::default(),
      depreciation: None,
      best_before: None,
      divisible_amount: None,
      lock: Lock::default(),
      history: Vec::new(),
      created_at: Utc::now(),
      created_by: 0,
    }
  }
}

impl VecPackMember for Upl {
  type Out = String;

  fn get_id(&self) -> &Self::Out {
    &self.id
  }
}
