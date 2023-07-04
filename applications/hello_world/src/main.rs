use regex::Regex;

fn main() {
    let date: &str = "2023-07-05";
    println!("Did our date match? {}", check_date(date));
}

fn check_date(date: &str) -> bool {
    let re: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}

#[cfg(test)]
mod tests_check_date {
    use crate::*; // or `use super::*;`

    #[test]
    fn happy_path_1() {
        let date: &str = "2023-07-05";
        assert_eq!(true, check_date(date), "should be match YYYY-MM-DD");
    }

    #[test]
    fn unhappy_path_1() {
        let date: &str = "20230705";
        assert_eq!(false, check_date(date), "should NOT be match YYYYMMDD");
    }
}
