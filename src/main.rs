use std::path::{Path, PathBuf};

use archive::ArchiveObject;
use futures::lock::Mutex;
use packman::*;
use reservation::Reservation;
use serde::Serialize;

mod archive;
mod id;
mod index;
mod prelude;
mod reservation;
mod upl;

pub use id::*;
use upl::Upl;

struct UplService {
  // Add | Get
  // Implemented under Mutex, so there is no FS race condition
  index: Mutex<index::UplIndex>,
  // AddReservation | AddUpl | Increase | Descrease | ClearCart
  reservation: Mutex<Pack<Vec<Reservation>>>,
  // UPL locations
  locations: Mutex<Pack<Vec<()>>>,
  // Create | Move | ..
  upls: Mutex<VecPack<Upl>>,
  // New | Restore | Get
  archive: Mutex<archive::ArchiveStore>,
}

impl UplService {
  fn init(
    index: index::UplIndex,
    reservation: Pack<Vec<Reservation>>,
    locations: Pack<Vec<()>>,
    upls: VecPack<Upl>,
    archive: archive::ArchiveStore,
  ) -> Self {
    Self {
      index: Mutex::new(index),
      reservation: Mutex::new(reservation),
      locations: Mutex::new(locations),
      upls: Mutex::new(upls),
      archive: Mutex::new(archive),
    }
  }
  // Archive UPL
  fn archive(&self, upl_id: u32) {}
  // Restore UPL from archive
  fn restore(&self, upl_id: u32) {}
  // Find UPL in archive
  fn find_archive(&self, upl_id: u32) {}
}

fn main() {
  // Init UPL Index
  let upl_index = index::UplIndex::init(PathBuf::from("data/upl_index"));

  // Init Reservation DB
  let reservation: Pack<Vec<Reservation>> =
    Pack::load_or_init(PathBuf::from("data"), "reservation")
      .expect("Error while loading reservation database");

  // Init Locations DB
  let locations: Pack<Vec<()>> = Pack::load_or_init(PathBuf::from("data"), "locations")
    .expect("Error while loading locations db");

  // Init UPL DB
  let upls: VecPack<Upl> =
    VecPack::load_or_init(PathBuf::from("data/upls")).expect("Error while loading UPL database");

  // Init UPL Archive
  // All the sold UPLs are stored here
  let upl_archive = archive::ArchiveStore::init(PathBuf::from("data/upl_archive"));

  // Create UplService
  let upl_service: UplService =
    UplService::init(upl_index, reservation, locations, upls, upl_archive);

  // RPC code goes here
}
