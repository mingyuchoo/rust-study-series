//! # add_one
//!
//! `add_one` 은 일부 연산을 더 쉽게 하기 위한 유틸리티 모음입니다.

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_add_one() {
    assert_eq!(add_one(1), 2);
  }
}

/// 주어진 숫자에 1을 더합니다.
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
/// assert_eq!(answer, 6);
/// ```
/// # Panics
///
/// # Errors
///
/// # Safety
pub fn add_one(x: i32) -> i32 {
  x + 1
}
