use crate::upl::*;

pub enum ServiceError {
  InternalError(String),
  NotFound(String),
  AlreadyExists(String),
  BadRequest(String),
}

impl ServiceError {
  pub fn internal_error(msg: &str) -> Self {
    ServiceError::InternalError(msg.to_string())
  }
  pub fn not_found(msg: &str) -> Self {
    ServiceError::NotFound(msg.to_string())
  }
  pub fn already_exist(msg: &str) -> Self {
    ServiceError::AlreadyExists(msg.to_string())
  }
  pub fn bad_request(msg: &str) -> Self {
    ServiceError::BadRequest(msg.to_string())
  }
}

impl std::fmt::Display for ServiceError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ServiceError::InternalError(msg) => write!(f, "{}", msg),
      ServiceError::NotFound(msg) => write!(f, "{}", msg),
      ServiceError::AlreadyExists(msg) => write!(f, "{}", msg),
      ServiceError::BadRequest(msg) => write!(f, "{}", msg),
    }
  }
}

impl std::fmt::Debug for ServiceError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("")
      .field(&"ServiceError".to_string())
      .field(self)
      .finish()
  }
}

impl From<ServiceError> for ::tonic::Status {
  fn from(error: ServiceError) -> Self {
    match error {
      ServiceError::InternalError(msg) => ::tonic::Status::internal(msg),
      ServiceError::NotFound(msg) => ::tonic::Status::not_found(msg),
      ServiceError::AlreadyExists(msg) => ::tonic::Status::already_exists(msg),
      ServiceError::BadRequest(msg) => ::tonic::Status::invalid_argument(msg),
    }
  }
}

impl From<::packman::PackError> for ServiceError {
  fn from(error: ::packman::PackError) -> Self {
    match error {
      ::packman::PackError::ObjectNotFound => ServiceError::not_found(&error.to_string()),
      _ => ServiceError::internal_error(&error.to_string()),
    }
  }
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl From<std::env::VarError> for ServiceError {
  fn from(error: std::env::VarError) -> Self {
    ServiceError::internal_error(&format!("ENV KEY NOT FOUND. {}", error))
  }
}

use gzlib::proto::upl::upl_obj;

impl From<Kind> for upl_obj::Kind {
  fn from(kind: Kind) -> Self {
    match kind {
      Kind::Sku { sku } => Self::Sku(upl_obj::KindSku { sku: sku }),
      Kind::BulkSku { sku, upl_pieces } => Self::BulkSku(upl_obj::KindBulkSku { sku, upl_pieces }),
      Kind::OpenedSku {
        sku,
        amount,
        successors,
      } => Self::OpenedSku(upl_obj::KindOpenedSku {
        sku,
        amount,
        successors,
      }),
      Kind::DerivedProduct {
        derived_from,
        derived_from_sku: _,
        amount,
      } => Self::DerivedProduct(upl_obj::KindDerivedProduct {
        derived_from,
        amount,
      }),
    }
  }
}

impl From<Lock> for upl_obj::Lock {
  fn from(lock: Lock) -> Self {
    match lock {
      Lock::Cart(cart_id) => Self::CartLock(cart_id),
      Lock::Delivery(delivery_id) => Self::DeliveryLock(delivery_id),
      Lock::Inventory(inventory_id) => Self::InventoryLock(inventory_id),
      Lock::None => Self::None(()),
    }
  }
}

impl From<Location> for upl_obj::Location {
  fn from(location: Location) -> Self {
    match location {
      Location::Stock(stock_id) => Self::Stock(stock_id),
      Location::Delivery(delivery_id) => Self::Delivery(delivery_id),
      Location::Cart(cart_id) => Self::Cart(cart_id),
      Location::Discard(discard_id) => Self::Discard(discard_id),
    }
  }
}

impl From<Upl> for gzlib::proto::upl::UplObj {
  fn from(upl: Upl) -> Self {
    Self {
      id: upl.id.clone(),
      product_id: upl.product_id,
      sku_id: upl.get_sku(),
      product_unit: upl.product_unit.clone(),
      upl_piece: upl.get_upl_piece(),
      is_healty: upl.is_available_healthy(),
      best_before: match upl.best_before {
        Some(bbefore) => bbefore.to_rfc3339(),
        None => "".to_string(),
      },
      depreciation: match &upl.depreciation {
        Some(dp) => Some(upl_obj::Depreciation {
          depreciation_id: dp.depreciation_id,
          depreciation_comment: dp.comment.clone(),
        }),
        None => None,
      },
      procurement_id: upl.procurement_id,
      procurement_net_price: upl.procurement_net_price,
      procurement_net_price_sku: upl.procurement_net_price_sku,
      is_divisible: upl.is_divisible(),
      sku_divisible_amount: upl.get_divisible_amount().unwrap_or(0),
      kind: Some(upl.kind.clone().into()),
      lock: Some(upl.lock.clone().into()),
      location: Some(upl.location.clone().into()),
      has_special_price: upl.get_upl_has_special_price(),
      price_net: match upl.get_upl_special_price_net() {
        Some(spn) => spn,
        None => upl.price_net,
      },
      vat: upl.vat.to_string(),
      price_gross: match upl.get_upl_special_price_net() {
        Some(spn) => spn * upl.vat,
        None => upl.price_gross,
      },
      margin_net: match upl.get_upl_special_price_margin() {
        Some(sm) => sm,
        None => upl.margin_net,
      },
      is_archived: false,
      created_by: upl.created_by,
      created_at: upl.created_at.to_rfc3339(),
    }
  }
}
