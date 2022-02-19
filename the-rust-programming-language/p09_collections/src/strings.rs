pub fn call1() {
    let mut s1 = String::new();
    println!("{}", s1);

    let data = "문자열 초깃값";
    let s2 = data.to_string();
    println!("{}", s2);

    // 문자열 리터럴의 to_string() 메서드를 직접 호출할 수 있다.
    let s3 = "문자열 초깃값".to_string();
    println!("{}", s3);

    let s4 = String::from("문자열 초깃값");
    println!("{}", s4);
}

pub fn call2() {
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("{}", s1);

    let s2 = "baz";
    s1.push_str(s2);
    println!("s2: {}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3:{}", s3);
}

pub fn call3() {
    let s1 = String::from("Hello, ");

    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    // 이렇게 하면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다.
    // println!("{}", s1);

    println!("{}", s2);
    println!("{}", s3);
}

pub fn call4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    // 이렇게 하면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다.
    //println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);

    let s1 = String::from("tic");
    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s5);
}

pub fn call5() {
    let s1 = String::from("hello");
    // let h = s1[0];

    let len1 = String::from("Hola").len();
    let len2 = String::from("안녕하세요").len();
    println!("len1: {}, len2: {}", len1, len2);

    let hello = "안녕하세요";
    let s2 = &hello[0..3];
    println!("s2: {}", s2);
}

pub fn call6() {
    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for c in "안녕하세요".bytes() {
        println!("{}", c);
    }
}
