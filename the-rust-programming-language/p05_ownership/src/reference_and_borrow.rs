pub fn return_value() {
    println!("-- return_value()");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{s1}'의 길이는 {len}입니다.");

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let mut s2 = String::from("hello");
    println!("address of     s2: {:p}", &s2);
    println!("address of mut s2: {:p}", &mut s2);

    change(&mut s2);
    println!("{s2}");

    fn change(some_string: &mut String) {
        println!(
            "some_string: {some_string}, address of some_string: \
                  {some_string:p}"
        );
        some_string.push_str(", world");
    }
}

pub fn mutable_reference() {
    println!("-- mutable_reference()");

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{r1}");
    }
    let r2 = &mut s;

    println!("{r2}");
}

pub fn dead_reference() {
    let referend_to_nothing = dangle();
    println!("{referend_to_nothing}");

    // fn dangle() -> &String {      // bad
    fn dangle() -> String {
        // good
        let s = String::from("hello");
        s
    }
}
