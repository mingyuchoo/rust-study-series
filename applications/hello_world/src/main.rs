use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let date: &str = "2023-07-05";
    println!("Did our date match? {}", check_date(date).expect("Error!"));

    Ok(())
}

fn check_date(date: &str) -> Result<bool, regex::Error> {
    match Regex::new(r"^\d{4}-\d{2}-\d{2}$") {
        Ok(re) => Ok(re.is_match(date)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests_check_date {
    use crate::*; // or `use super::*;`

    #[test]
    fn happy_path_1() {
        let date: &str = "2023-07-05";
        assert_eq!(
            true,
            check_date(date).expect("Error!"),
            "should be match YYYY-MM-DD"
        );
    }

    #[test]
    fn unhappy_path_1() {
        let date: &str = "20230705";
        assert_eq!(
            false,
            check_date(date).expect("Error!"),
            "should NOT be match YYYYMMDD"
        );
    }
}
