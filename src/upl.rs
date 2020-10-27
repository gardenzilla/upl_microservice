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
use crate::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Upl {
  // Unique UPL ID
  // i32 for the better inter
  // service communication
  id: i32,
  // Related SKU
  // SKU maybe should be i32 as well?
  sku: String,
  procurement_id: i32,
  procurement_net_price: f32,
}

pub struct UplId(u32);

impl UplId {
  fn from_i32() -> Self {
    unimplemented!()
  }
}

impl Upl {
  pub fn new(id: UplId) -> Self {
    unimplemented!()
  }
}
