use packman::*;

pub mod id;
pub mod prelude;
pub mod upl;

pub use id::*;

fn main() {
  let num = UplId::new(19);
  println!("{:?}", is_valid(num));
}
