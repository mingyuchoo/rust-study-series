use add_one;
use art::{mix,
          PrimaryColor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = 10;
    println!("안녕하세요? {} 더하기 1은 {}입니다!",
             num,
             add_one::add_one(num));

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);

    Ok(())
}
