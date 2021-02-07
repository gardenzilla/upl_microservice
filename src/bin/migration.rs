use std::path::PathBuf;

use migration::upl_old::UplMethods as OldUplMethods;
use packman::*;
use upl_microservice::{migration, upl::UplMethods as NewUplMethods};

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
  let mut new_upl_db: VecPack<upl_microservice::upl::Upl> =
    VecPack::load_or_init(PathBuf::from("data/migration/new/upls"))
      .expect("Error while loading new UPLs db");

  // Init new UPL DB Archive
  let mut new_upl_db_archive: VecPack<upl_microservice::upl::Upl> =
    VecPack::load_or_init(PathBuf::from("data/migration/new/upl_archive"))
      .expect("Error while loading new UPLs db");

  let mut counter = 0;

  // Convert old UPL to new UPL
  for old_upl in old_upl_db.iter() {
    let ou = old_upl.unpack();

    println!("Starting UPL {}", ou.id.to_string());

    let sku_id = match &ou.kind {
      migration::upl_old::Kind::Sku { sku } => *sku,
      migration::upl_old::Kind::BulkSku { sku, upl_pieces: _ } => *sku,
      migration::upl_old::Kind::OpenedSku {
        sku,
        amount,
        successors,
      } => *sku,
      migration::upl_old::Kind::DerivedProduct {
        derived_from,
        amount,
      } => panic!("DerivedProduct FOUND! IMPOSSIBLE"),
    };

    let sku = sku_db
      .find_id(&sku_id)
      .expect(&format!("Cannot found SKU with ID {}", sku_id))
      .unpack()
      .clone();

    let price = price_db
      .find_id(&sku_id)
      .expect(&format!("Cannot found PRICE for SKU with ID {}", sku_id))
      .unpack()
      .clone();

    {
      use upl_microservice::upl::UplMethods;

      let new_upl = upl_microservice::upl::Upl::new(
        ou.id.to_string(),
        ou.product_id,
        sku.unit.to_string(),
        sku_id,
        match ou.get_kind() {
          migration::upl_old::Kind::OpenedSku {
            sku: _,
            amount: _,
            successors: _,
          } => ou
            .get_divisible_amount()
            .expect("Error getting divisible amount for bulk sku"),
          _ => ou.get_upl_piece(),
        },
        sku.get_divisible_amount(),
        sku.can_divide,
        price.net_retail_price,
        upl_microservice::upl::VAT::from_str(&price.vat.to_string()).expect("Error converting VAT"),
        ou.procurement_id,
        ou.procurement_net_price,
        match &ou.location {
          migration::upl_old::Location::Stock(id) => upl_microservice::upl::Location::Stock(*id),
          migration::upl_old::Location::Delivery(id) => {
            upl_microservice::upl::Location::Delivery(*id)
          }
          migration::upl_old::Location::Cart(id) => {
            upl_microservice::upl::Location::Cart(id.to_string())
          }
          migration::upl_old::Location::Discard(id) => {
            upl_microservice::upl::Location::Discard(*id)
          }
        },
        ou.best_before,
        match ou.kind {
          migration::upl_old::Kind::OpenedSku {
            sku: _,
            amount: _,
            successors: _,
          } => true,
          migration::upl_old::Kind::DerivedProduct {
            derived_from: _,
            amount: _,
          } => true,
          _ => false,
        },
        ou.created_by,
      )
      .expect(&format!("UPL nem hozható létre! ID: {}", ou.id));

      match new_upl.location {
        // Insert to archive as its sold
        upl_microservice::upl::Location::Cart(_) => {
          new_upl_db_archive.insert(new_upl).expect(&format!(
            "Cannot insert new UPL into new DB with ID: {}",
            ou.id.to_string()
          ));
        }
        // Insert to UPL DB
        _ => {
          new_upl_db.insert(new_upl).expect(&format!(
            "Cannot insert new UPL into new DB with ID: {}",
            ou.id.to_string()
          ));
        }
      }

      counter += 1;
      println!("{}", counter);
    }
  }
}
