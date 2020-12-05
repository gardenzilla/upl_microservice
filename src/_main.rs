use packman::*;
use serde::Serialize;

pub mod id;
pub mod prelude;
pub mod reservation;
pub mod upl;

pub use id::*;

fn filter<'a, T: 'a, F, I>(data: I, pagination: Pagination, filter: F) -> PaginatedObject<T>
where
  T: std::cmp::Ord + Serialize + Sized + Clone,
  I: IntoIterator<Item = &'a T>,
  F: FnMut(&&'a T) -> bool,
{
  // Apply filter
  let mut res = data.into_iter().filter(filter).collect::<Vec<&T>>();

  // Apply Order
  match pagination.order {
    Order::Asc => res.sort(),
    Order::Desc => res.sort(),
    Order::None => (),
  }

  // Apply pagination and create result object
  PaginatedObject {
    data: res
      .iter()
      .skip(pagination.page_size * (pagination.current_page - 1))
      .take(pagination.page_size)
      .map(|i| (*i).clone())
      .collect::<Vec<T>>(),
    page_size: pagination.page_size,
    current_page: pagination.current_page,
    prev_page: pagination.current_page - 1,
    next_page: pagination.current_page + 1,
  }
}

enum Order {
  None,
  Asc,
  Desc,
}

struct Pagination {
  order: Order,
  page_size: usize,
  current_page: usize,
}

#[derive(Debug)]
struct PaginatedObject<T>
where
  T: Serialize,
{
  data: Vec<T>,
  page_size: usize,
  current_page: usize,
  prev_page: usize,
  next_page: usize,
}

fn demo() {
  struct Reservation {
    cart: u32,
    divided: bool,
    amount_requested: u32,
    amount_taken: u32,
  }

  struct SkuReservation {
    sku: u32,
    global_reservation: Vec<Reservation>,
    local_reservation: Vec<Reservation>,
  }

  let reservations: Vec<SkuReservation> = Vec::new();
}

fn main() {
  // let num = UplId::new(19);
  // println!("{:?}", is_valid(num));
  // Sort process
  // 1) Filter
  // 2) Order
  // 3) Pagination
  let a = vec![1, 6, 3, 7, 5, 8, 2, 9];
  let mut b = a.iter().filter(|i| **i > 3).collect::<Vec<&i32>>();
  b.sort();
  println!("A is {:?}", a);
  println!("B is {:?}", b.chunks(2).collect::<Vec<&[&i32]>>());
  println!(
    "B page 2 with size 2 is {:?}",
    b.iter().skip(2).take(2).collect::<Vec<&&i32>>()
  );

  let f = |i: &&i32| **i > 3;
  let pager = Pagination {
    order: Order::Asc,
    page_size: 2,
    current_page: 1,
  };
  println!("Paginated result is {:?}", filter(a.iter(), pager, f));

  // Determine UPL index path parts from UPL
  // This kind of partinioning enable us to store safily
  // millions of UPLs without crashing the FS.
  // Maximum 1_000 folder per folder and maximum 1_000 index file
  // per folder.
  let get_upl_ipath = |u: u32| (u / 1_000_000, u % 1_000_000 / 1000, u % 1000);

  struct UplIndex {
    upl: u32,
    product: u32,
    sku: u32,
    created_at_epoch_utc: u32, // unix epoch
  }

  println!("Remainder [1_000] of 127 is {}", 127 / 1_000);
  println!("Remainder [1_000_000] of 127 is {}", 127 / 1_000_000);

  println!("Result is {:?}", get_upl_ipath(19));
  println!("Result is {:?}", get_upl_ipath(1_500));
  println!("Result is {:?}", get_upl_ipath(1_339));
  println!("Result is {:?}", get_upl_ipath(783_500));
  println!("Result is {:?}", get_upl_ipath(12_157_500));
  println!("Result is {:?}", get_upl_ipath(12_157_319));
}
