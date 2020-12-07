use serde::{Deserialize, Serialize};
use std::ops::Deref;

// 7    => 773
// 9    => 991
// 10   => 2108
// 49   => 7493

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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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

impl Deref for UplId {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_calculate_check_from_vec() {
    assert_eq!(calculate_check_from_vec(vec![0]), (9, 0));
    assert_eq!(calculate_check_from_vec(vec![1]), (1, 9));
    assert_eq!(calculate_check_from_vec(vec![2]), (2, 8));
    assert_eq!(calculate_check_from_vec(vec![3]), (3, 7));
    assert_eq!(calculate_check_from_vec(vec![4]), (4, 6));
    assert_eq!(calculate_check_from_vec(vec![5]), (5, 5));
    assert_eq!(calculate_check_from_vec(vec![6]), (6, 4));
    assert_eq!(calculate_check_from_vec(vec![7]), (7, 3));
    assert_eq!(calculate_check_from_vec(vec![8]), (8, 2));
    assert_eq!(calculate_check_from_vec(vec![9]), (9, 1));
    assert_eq!(calculate_check_from_vec(vec![4, 5]), (3, 7));
  }

  #[test]
  fn test_calculate_check() {
    assert_eq!(calculate_check(0), (9, 0));
    assert_eq!(calculate_check(1), (1, 9));
    assert_eq!(calculate_check(459), (1, 9));
    assert_eq!(calculate_check(311), (2, 8));
  }

  #[test]
  fn test_new() {
    assert_eq!(UplId::new(0), 900);
    assert_eq!(UplId::new(1), 119);
    assert_eq!(UplId::new(2), 228);
    assert_eq!(UplId::new(3), 337);
    assert_eq!(UplId::new(4), 446);
    assert_eq!(UplId::new(5), 555);
    assert_eq!(UplId::new(9), 991);
    assert_eq!(UplId::new(10), 2108);
    assert_eq!(UplId::new(49), 7493);
    assert_eq!(UplId::new(99), 7993);
    assert_eq!(UplId::new(1758), 317587);
  }

  #[test]
  fn test_validation() {
    assert_eq!(is_valid(119), true);
    assert_eq!(is_valid(218), false);
    assert_eq!(is_valid(199), false);
    assert_eq!(is_valid(2108), true);
    assert_eq!(is_valid(7993), true);
    assert_eq!(is_valid(6994), false);
    assert_eq!(is_valid(92349671), true);
    assert_eq!(is_valid(82349672), false);
    assert_eq!(is_valid(900), true);
    assert_eq!(is_valid(7493), true);
  }

  #[test]
  fn test_uniqueness() {
    use std::collections::HashSet;
    let mut num_vec: HashSet<u32> = HashSet::new();
    let mut _num_vec: Vec<u32> = Vec::new();
    for n in 0..100_000 {
      _num_vec.push(n);
    }
    let res = _num_vec.iter().all(move |i| num_vec.insert(*i));
    assert_eq!(res, true);
  }
}
