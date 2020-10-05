use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type StockId = String;
pub type CartId = String;
pub type CustomerId = String;
pub type UplId = String;
pub type SKU = String;
pub type ProcurementId = String;
pub type UserId = String;

enum UplLocation {
  Stock(StockId),
  //Delivery(DeliveryId),
  Cart(CartId),
  Purchase(CartId),
}

enum Status {
  UnOpened,
  Opened(u32),
}

type ScrapingId = String;

pub struct Upl {
  id: UplId,
  sku: SKU,
  procurement_id: ProcurementId,         // todo: maybe ProcurementId?
  net_procurement_price: f32,            // todo: sure?
  net_retail_price: f32,                 // ?
  net_retail_price_custom: Option<f32>,  // ?
  custom_price_history: Vec<UplCPH>,     // ?
  can_divide: Option<u32>,               // ?
  best_before: Option<DateTime<Utc>>,    // todo: DateTime<Utc>!
  location_history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
  inherited_from: Option<UplId>,         // If it's derived from another UPL
  is_opened: bool,                       // If it's opened
  has_damage: Option<ScrapingId>,        // If damaged, we need to enlcose the related scrape doc id
}

pub struct UplCPH {
  created_at: DateTime<Utc>,
  created_by: UserId,
  net_retail_price_custom: f32,
}

pub struct UplHistoryItem {
  created_at: DateTime<Utc>,
  created_by: UserId,
  location: UplLocation,
}

pub mod New {
  use crate::prelude::*;
  use std::collections::HashMap;
  type SKU = String;
  pub struct UplPhdr {
    upl_id: u32,
    sku: SKU,
  }

  enum LocationKind {
    Stock,
    Cart,
    Delivery,
    Customer,
  }
  struct Location {
    id: u32,
    kind: LocationKind,
    upl_store: HashMap<SKU, Vec<UplPhdr>>,
  }

  struct UplLocations {
    stock: Vec<Location>,
    cart: Vec<Location>,
    delivery: Vec<Location>,
  }

  impl UplLocations {
    pub fn move_upl(
      &mut self,
      from_kind: &str,
      from_id: u32,
      to_kind: &str,
      to_id: u32,
      sku: &SKU,
      upl_id: u32,
    ) -> ServiceResult<()> {
      let mut from_place = match from_kind {
        "stock" => &mut self.stock,
        "cart" => &mut self.cart,
        "delivery" => &mut self.delivery,
        _ => return Err(ServiceError::not_found("Given from kind not found")),
      };
      let mut to_place = match to_kind {
        "stock" => &mut self.stock,
        "cart" => &mut self.cart,
        "delivery" => &mut self.delivery,
        _ => return Err(ServiceError::not_found("Given to kind not found")),
      };
      Ok(())
    }
  }
}
