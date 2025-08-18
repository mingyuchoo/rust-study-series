//! This is an example application for starter
use regex::Regex;

/// check date format and print out stdout
///
/// # Arguments
///
/// * No argument
///
/// # Examples
fn main() -> Result<(), std::io::Error> {
    // "2023-07-05" is stored in code section
    // data is stored in stack frame
    // data stores reference of address of "2023-07-05" stored in code section
    let date: &str = "2023-07-05";

    println!("Did our date match? {}", check_date(date).expect("Error!"));

    Ok(())
}

/// Check date
///
/// # Arguments
///
/// * `date` - date string to check
///
/// # Examples
///
/// ```
/// let ret = check_date("2023-01-01"); 
/// ```
///
/// # Descriptions
///
/// `date` parameter variable is stored in stack frame in memory
/// `r#^\d{4}-d{2}-\d{2}$"` is stored in code section (a.k.a text section)
fn check_date(date: &str) -> Result<bool, regex::Error> {
    match Regex::new(r"^\d{4}-\d{2}-\d{2}$") {
        | Ok(re) => Ok(re.is_match(date)),
        | Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests_check_date {
    use crate::*; // or `use super::*;`

    #[test]
    fn happy_path_1() {
        // `date` local variable is stored in stack frame
        // "2023-07-05" is stored in code section (a.k.a. text section)
        let date: &str = "2023-07-05";

        assert_eq!(true, check_date(date).expect("Error!"), "should be match YYYY-MM-DD");
    }

    #[test]
    fn unhappy_path_1() {
        let date: &str = "20230705";
        assert_eq!(false, check_date(date).expect("Error!"), "should NOT be match YYYYMMDD");
    }
}
