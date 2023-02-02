#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(2, add_two(2));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
