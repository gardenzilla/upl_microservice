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

pub struct Upl {
  id: UplId,
  sku: SKU,
  procurement_id: ProcurementId,         // todo: maybe ProcurementId?
  net_procurement_price: f32,            // todo: sure?
  net_retail_price: f32,                 // ?
  net_retail_price_custom: Option<f32>,  // ?
  custom_price_history: Vec<UplCPH>,     // ?
  can_devide: Option<u32>,               // ?
  best_before: Option<DateTime<Utc>>,    // todo: DateTime<Utc>!
  location_history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
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
