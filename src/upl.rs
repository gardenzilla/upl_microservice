use crate::id;
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
 * [ ] VecPack<Upl>
 * [ ] Location index
 * [ ] Create new location
 * [ ] Move between location
 * [ ] Location history log
 * [ ] Best before date
 * [ ] Procurement ID
 * [ ] Devide
 * [ ] Merge
 * [ ] UPL_ID library (create from i32; validate)
 * [ ] Set Price
 * [ ] Set culling price (selejtezés általi árcsökkenés)
 *
 * Locations:
 * - Stock
 * - Cart/Purchase
 *
 * InHouse / Sold/Out
 *    *         *
 *     \        |
 *      * Manage|updates & lookup  
 *              |
 *              * No updates at all (LOCK)
 *
 * Követelmény:
 *  - UPL ID alapján egy időintervallumon belül bármikor megtalálható legyen
 *  - Aktív UPL-ek helyszín alapján lekérhetőek legyenek
 *  - UPL-t lehessen helyszínek között mozgatni
 *  - Már kosárba tett UPL ne legyen frissíthető
 *  - Már kosárba tett UPL megtalálható legyen, de ne "kezeljük"
 *  - Egy UPL-ről tudnunk kell, hogy már el van-e adva
 *
 * UPL és egyéb kapcsolatok:
 *  - Beszerző modul-ból érkeznek
 *  - Selejtező modul selejtezést tud bejegyezni
 *  - Raktár modul le tud kérni raktár készletet
 *  - Kosár modul a kosár tartalmát tudja lekérni, de ő saját listát is vezet
 *  - Árazás modul le tudja kérdezni a raktáron lévő termékek "hasznait" és a már eladott
 *    termékek "hasznait"
 */

pub enum Location {
  Cart(u32),
  Stock(u32),
}

/// VAT enum
/// represents the internally used VAT variants.
/// Currently based on the Hungarian TAX law.
// TODO: Implement this based on the pricing module
pub enum Vat {
  /// Percentage based VAT
  /// such as 27%, 5%, etc..
  /// It also could be _27, _5, AAM, FAD
  /// and implement a to string conversion
  /// in the way the cart and invoice modules need
  Percentage(u32),
  /// Alanyi adómentes
  AAM,
  FAD,
}

pub enum Quantity {
  Simple(u32),
  Complex(u32, u32),
  Partial(u32),
}

pub enum Unit {
  Piece,
  Mm,
  Ml,
  Gram,
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
  // Related SKU
  // SKU maybe should be i32 as well?
  sku: String,
  // * Procurement
  procurement_id: i32,
  // Net wholesale price in which
  // this item was purchased by us
  procurement_net_price: f32,
  // Current UPL location
  location: Location,
  // Previous locations
  // history
  location_history: Vec<Location>,
  // Applied retail VAT
  vat: Option<Vat>,
  // Retail net price
  // Currently applied net
  // retail price for this product
  // Optional, as a registered UPL
  // doesn't need to be priced to sell
  // Mainly a newly registered UPL, after
  // the procurement process and before the price
  // validation process.
  retail_net_price: Option<f32>,
  // If the product is injured
  // it should be scraped. This field
  // contains the related scrap id
  scrap_id: Option<u32>, // TODO: scrap_price_log?
  // Related scrap comment
  // if there any
  scrap_comment: Option<String>,
  // Related scrap price
  // if there any
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
  quantity: Quantity,
  // Product unit
  // Inherited from Product(service)
  unit: Unit,
  // If a SKU is divisible, this field is set ture.
  // Inherited from the related SKU's / Product(service)
  // UPL can only be divided if this is true.
  // If a UPL is already divided, related SKU update wont
  // affect it.
  divisible: bool,
  // Lock enum
  // When a UPL is locked by any reason,
  // that UPL cannot be updated.
  lock: Lock,
  // Userid who created
  created_by: String,
  // Utc datetime when this object
  // created
  date_created: DateTime<Utc>,
  // Experimental field
  // true when we expect the Upl object
  // to be outdated.
  // e.g.: returning after a locked period
  dirty: bool,
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
}
