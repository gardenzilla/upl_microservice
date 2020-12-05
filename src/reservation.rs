// Copyright (C) 2020 Peter Mezei
//
// This file is part of Gardenzilla.
//
// Gardenzilla is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Gardenzilla is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Gardenzilla.  If not, see <http://www.gnu.org/licenses/>.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// Reservation storage
// Itt tároljuk a
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Scope {
  // Local(StoreId)
  // Local means a stock reservation
  Local(u32),
  // Global scope means
  // we have a reservation over all the locations
  Global,
}

impl Default for Scope {
  fn default() -> Self {
    Self::Global
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReservationFor {
  // We have a reservation to an exact SKU
  Sku(u32),
  // We have a reservation to a divided
  // product. The related amount
  DividedProduct(u32),
}

impl Default for ReservationFor {
  fn default() -> Self {
    Self::Sku(0)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reservation {
  // Cart ID that owns this reservation
  cart_id: u32,
  // Sku or Product ReservationFor
  reservation_for: ReservationFor,
  // Local or global scope
  // Local means reservation in a given stock
  // Global means over all locations
  scope: Scope,
  // Amount to be reserved
  // It means amount of SKU when its for Sku
  // It means divided amount of a Product when its for a DividedProduct
  reserved_amount: u32,
  // Already taken amount
  // Remaining amount will be calculated
  // by reserved_amount - already_taken
  already_taken: u32,
  // Reservation creation time
  created_at: DateTime<Utc>,
  // Reservation is created by
  // Should be the same as the user who
  // is working with the cart
  created_by: String,
}

impl Default for Reservation {
  fn default() -> Self {
    Self {
      cart_id: 0,
      reservation_for: ReservationFor::default(),
      scope: Scope::default(),
      reserved_amount: 0,
      already_taken: 0,
      created_at: Utc::now(),
      created_by: "".into(),
    }
  }
}
