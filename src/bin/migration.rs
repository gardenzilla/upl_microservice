use std::path::PathBuf;

use packman::*;
use upl_microservice::migration;

fn main() {
  // Init UPL DB
  let old_upl_db: VecPack<migration::upl_old::Upl> =
    VecPack::load_or_init(PathBuf::from("data/migration/old/upls"))
      .expect("Error while loading old UPLs db");

  // Init SKU DB
  let sku_db: VecPack<migration::product::Sku> =
    VecPack::load_or_init(PathBuf::from("data/migration/old/skus"))
      .expect("Error while loading SKUs db");

  // Init Pricing DB
  let price_db: VecPack<migration::price::Sku> =
    VecPack::load_or_init(PathBuf::from("data/migration/old/prices"))
      .expect("Error while loading Prices db");

  // Init new UPL DB
  let new_upl_db: VecPack<upl_microservice::upl::Upl> =
    VecPack::load_or_init(PathBuf::from("data/migration/new/upls"))
      .expect("Error while loading new UPLs db");

  // Convert old UPL to new UPL
}
