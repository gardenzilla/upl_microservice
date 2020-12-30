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
use packman::VecPackMember;
use serde::{Deserialize, Serialize};

pub trait ReservationMethods
where
  Self: Sized,
{
  /// Create new reservation object
  fn new(cart_id: u32, subject: Subject, scope: Scope, reserved_amount: u32) -> Self;
  /// Get cart id ref
  fn get_cart_id(&self) -> &u32;
  /// Get subject ref
  fn get_subject(&self) -> &Subject;
  /// Get scope ref
  fn get_scope(&self) -> &Scope;
  /// Get amount reserved ref
  fn get_amount_reserved(&self) -> &u32;
  /// Get amount already taken ref
  fn get_amount_taken(&self) -> &u32;
  /// Set amount reserved
  fn set_amount_reserved(&mut self, amount: u32) -> &Self;
  /// Set amount taken
  fn set_amount_taken(&mut self, amount: u32) -> &Self;
}

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
pub enum Subject {
  // We have a reservation to an exact SKU
  Sku(u32),
  // We have a reservation to a divided
  // product. The related amount
  DividedProduct(u32),
}

impl Default for Subject {
  fn default() -> Self {
    Self::Sku(0)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reservation {
  // Cart ID that owns this reservation
  cart_id: u32,
  // Sku or DividedProduct
  subject: Subject,
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
}

impl Default for Reservation {
  fn default() -> Self {
    Self {
      cart_id: 0,
      subject: Subject::default(),
      scope: Scope::default(),
      reserved_amount: 0,
      already_taken: 0,
    }
  }
}

impl ReservationMethods for Reservation {
  fn new(cart_id: u32, subject: Subject, scope: Scope, reserved_amount: u32) -> Self {
    Self {
      cart_id,
      subject,
      scope,
      reserved_amount,
      already_taken: 0,
    }
  }

  fn get_cart_id(&self) -> &u32 {
    &self.cart_id
  }

  fn get_subject(&self) -> &Subject {
    &self.subject
  }

  fn get_scope(&self) -> &Scope {
    &self.scope
  }

  fn get_amount_reserved(&self) -> &u32 {
    &self.reserved_amount
  }

  fn get_amount_taken(&self) -> &u32 {
    &self.already_taken
  }

  fn set_amount_reserved(&mut self, amount: u32) -> &Self {
    self.reserved_amount = amount;
    self
  }

  fn set_amount_taken(&mut self, amount: u32) -> &Self {
    self.already_taken = amount;
    self
  }
}
