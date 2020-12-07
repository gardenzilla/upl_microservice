// Luhn algorythm
// based on https://en.wikipedia.org/wiki/Luhn_algorithm
fn calc_check(u: u64) -> u8 {
  match 10 - (calc(u, 1) % 10) {
    x if x != 10 => x,
    _ => 0,
  }
}

fn make_id(u: u64) -> u64 {
  (u * 10 + calc_check(u) as u64) * 10 + (9 - calc_check(u) as u64)
}

fn calc(n: u64, i: u32) -> u8 {
  let digit = n % 10;
  // if odd
  let res = match i % 2 {
    0 => digit,
    _ => match digit * 2 {
      x if x > 9 => x - 9,
      x => x,
    },
  };
  // Should we continue?
  let left = n / 10;
  if left > 0 {
    // Then continute calling C but with i + 1
    return calc(left, i + 1) + res as u8;
  }
  res as u8
}

fn is_valid(n: u64) -> bool {
  (n % 10) == calc_check(n / 10) as u64
}

fn main() {
  (1..1_000)
    .into_iter()
    .for_each(|i| println!("{} => {}", i, make_id(i)));
  println!("OK");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_is_valid() {
    assert_eq!(is_valid(5652), true);
    assert_eq!(is_valid(5654), false);
    assert_eq!(is_valid(5653), false);
    assert_eq!(is_valid(5660), true);
  }
}
