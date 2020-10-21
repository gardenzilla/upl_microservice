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
 */
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum UplLocation {
    Stock(i32),
    Cart(i32),
}

pub struct Upl {
    id: i32,                                // UPL ID
    sku: String,                            // SKU (ID)
    procurement_id: Option<i32>,            // Procurement ID
    procurement_net_price: Option<f32>,     // net procurement price
    net_retail_price: Option<f32>,          // ?
    net_retail_price_culed: Option<f32>,    // ?
    can_divide: bool,                       // boolean operator
    best_before: Option<DateTime<Utc>>,     // todo: DateTime<Utc>!
    location_history: Vec<UplHistoryItem>,  // todo: + DateTime per event. Only location change?
    inherited_from: Option<i32>,            // If it's derived from another UPL
    is_opened: bool,                        // If it's opened
    has_damage: Option<(i32, String, f32)>, // Option<CullId, Description, culled net price>?
}

impl Upl {
    pub fn new(
        id: i32,
        sku: String,
        procurement_id: Option<i32>,
        procurement_net_price: Option<f32>,
        best_before: Option<DateTime<Utc>>,
        can_divide: bool,
    ) -> Self {
        Self {
            id,
            sku,
            procurement_id,
            procurement_net_price,
            net_retail_price: None,
            net_retail_price_culed: None,
            can_divide,
            best_before,
            location_history: Vec::new(),
            inherited_from: None,
            is_opened: false,
            has_damage: None,
        }
    }
}

pub struct UplHistoryItem {
    created_at: DateTime<Utc>,
    created_by: String,
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
