// xxx|x
// ---  \
//  |    validation number
//   \
//     UPL id inner 1 -> +infinite
fn calculate_check_from_vec(mut n: Vec<u32>) -> u32 {
  n.reverse();
  let remainder = 10
    - (n
      .iter()
      .enumerate()
      .fold(0, |acc, (index, number)| acc + *number * (index + 1) as u32)
      % 10);
  match remainder {
    10 => 0,
    _ => remainder,
  }
}

fn calculate_check(n: u32) -> u32 {
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
  if let Some((last_number, numbers)) = num_vec.split_last() {
    if calculate_check_from_vec(numbers.to_owned()) == *last_number {
      return true;
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
    let checksum = calculate_check(n);
    format!("{}{}", n.to_string(), checksum)
      .parse::<u32>()
      // I use unwrap or default as there is no
      // chance to run on error @petermezei
      .unwrap_or_default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_calculate_check_from_vec() {
    assert_eq!(calculate_check_from_vec(vec![0]), 0);
    assert_eq!(calculate_check_from_vec(vec![1]), 9);
    assert_eq!(calculate_check_from_vec(vec![2]), 8);
    assert_eq!(calculate_check_from_vec(vec![3]), 7);
    assert_eq!(calculate_check_from_vec(vec![4]), 6);
    assert_eq!(calculate_check_from_vec(vec![5]), 5);
    assert_eq!(calculate_check_from_vec(vec![6]), 4);
    assert_eq!(calculate_check_from_vec(vec![7]), 3);
    assert_eq!(calculate_check_from_vec(vec![8]), 2);
    assert_eq!(calculate_check_from_vec(vec![9]), 1);
    assert_eq!(calculate_check_from_vec(vec![4, 5]), 7);
  }

  #[test]
  fn test_calculate_check() {
    assert_eq!(calculate_check(0), 0);
    assert_eq!(calculate_check(1), 9);
    assert_eq!(calculate_check(459), 9);
    assert_eq!(calculate_check(311), 8);
  }

  #[test]
  fn test_new() {
    assert_eq!(UplId::new(0), 0);
    assert_eq!(UplId::new(1), 19);
    assert_eq!(UplId::new(2), 28);
    assert_eq!(UplId::new(3), 37);
    assert_eq!(UplId::new(4), 46);
    assert_eq!(UplId::new(5), 55);
    assert_eq!(UplId::new(9), 91);
    assert_eq!(UplId::new(10), 108);
    assert_eq!(UplId::new(99), 993);
    assert_eq!(UplId::new(1758), 17587);
  }

  #[test]
  fn test_validation() {
    assert_eq!(is_valid(19), true);
    assert_eq!(is_valid(18), false);
    assert_eq!(is_valid(99), false);
    assert_eq!(is_valid(108), true);
    assert_eq!(is_valid(993), true);
    assert_eq!(is_valid(994), false);
    assert_eq!(is_valid(2349671), true);
    assert_eq!(is_valid(2349672), false);
  }
}
