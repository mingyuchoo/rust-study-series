fn foo(_: i32, y: i32) {
    println!("이 함수 y 매개변수 사용한다:  {}", y);
}

pub fn call1() {
    foo(3, 4);
}

pub fn call2() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("이미 설정된 값을 덮어쓸 수 업습니다.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("현재 설정: {:?}", setting_value);
}

pub fn call3() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("일치하는 숫자: {}, {}, {}", first, third, fifth)
        }
    }
}

pub fn call4() {
    let _x = 5;
    let y = 10;
}

pub fn call5() {
    let s = Some(String::from("Hello"));

    // if let Some(_s) = s {  // ERROR
    if let Some(_) = s {
        println!("I found the string.");
    }
    println!("{:?}", s);
}

pub fn call6() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x={}", x),
    }
}

pub fn call7() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first={}, last={}", first, last);
        }
    }
}
