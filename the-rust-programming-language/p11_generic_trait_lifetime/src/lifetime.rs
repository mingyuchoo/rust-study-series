use std::fmt::Display;

pub fn call1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {result}");

    /*  CAN NOT complile this funciton
    fn longest(x: &str, y: &str) -> &str {
      if x.len() > y.len() {
        x
      } else {
        y
      }
    }
    */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

pub fn call2() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("문장에서 마침포'.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

pub fn call3() {
    // fn first_word<'a>(s: &'a str) -> &'a str {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    println!("{}", first_word("hello world"));
}

pub fn call4() {
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("주목하세요: {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "World");
    println!("더 긴 문자열은 {result} 입니다.");
}
