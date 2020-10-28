// 10-y|xxx|y
//   \  ---  \
//    *--|----validation number(s)
//        \
//         UPL id inner 1 -> +infinite
// 49 -> check number is (7 ,3)
// so 49 will be 7|49|3 => 7493
fn calculate_check_from_vec(mut n: Vec<u32>) -> (u32, u32) {
  n.reverse();
  let remainder = 10
    - (n
      .iter()
      .enumerate()
      .fold(0, |acc, (index, number)| acc + *number * (index + 1) as u32)
      % 10);
  match remainder {
    10 => (9, 0),
    _ => (10 - remainder, remainder),
  }
}

fn calculate_check(n: u32) -> (u32, u32) {
  let num_vec: Vec<u32> = n
    .to_string()
    .chars()
    .into_iter()
    .map(|c| c.to_digit(10).unwrap_or_default())
    .collect();
  calculate_check_from_vec(num_vec)
}

/// Validate a UplId predicate\
/// Check the validation number\
/// returns true if valid otherwise\
/// returns false
pub fn is_valid(n: u32) -> bool {
  let num_vec: Vec<u32> = n
    .to_string()
    .chars()
    .into_iter()
    .map(|c| c.to_digit(10).unwrap_or_default())
    .collect();
  if let Some((first_item, numbers)) = num_vec.split_first() {
    if let Some((last_number, numbers)) = numbers.split_last() {
      if calculate_check_from_vec(numbers.to_owned()) == (*first_item, *last_number) {
        return true;
      }
    }
  }
  false
}

pub struct UplId(u32);

impl UplId {
  /// Create new UPL from a given u32
  ///
  /// It will generate a checksum\
  /// and concanetate that 1 digit checksum\
  /// as the last digit.
  pub fn new(n: u32) -> u32 {
    let (checksum_first, checksum_last) = calculate_check(n);
    format!("{}{}{}", checksum_first, n.to_string(), checksum_last)
      .parse::<u32>()
      // I use unwrap or default as there is no
      // chance to run on error @petermezei
      .unwrap_or_default()
  }
}

fn main() {
  let mut v = Vec::new();
  for n in 0..1_000 {
    v.push(UplId::new(n));
  }
  v.sort();
  v.iter().for_each(|i| println!("{}", i));
}
