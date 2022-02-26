#[cfg(test)]
mod tests {
  use super::*;

  // `$ cargo test -- --ignored`
  #[test]
  #[ignore]
  #[should_panic(expected = "반드시 100보다 작거나 같은 값을 사용해야 합니다.")]
  fn greater_than_100() {
    Guess::new(200);
  }
}

pub struct Guess {
  value: u32,
}

impl Guess {
  pub fn new(value: u32) -> Guess {
    if value < 1 {
      panic!("반드시 1보다 크거나 같은 값을 사용해야 합니다. 지정한 값: {}", value);
    } else if value > 100 {
      panic!("반드시 100보다 작거나 같은 값을 사용해야 합니다. 지정한 값: {}", value);
    }
    Guess {
      value
    }
  }
}
