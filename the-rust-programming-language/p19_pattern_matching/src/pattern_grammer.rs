pub fn call1() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("others"),
    }
}

pub fn call2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(y)  => println!("match, y = {:?}", y),
        _        => println!("not match, x = {:?}", x),
    }
    println!("result: x= {:?}, y = {:?}", x, y);
}

pub fn call3() {
    let x = 1;

    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("others"),
    }
}

pub fn call4() {
    let x = 5;

    match x {
        1 ..= 5 => println!("one of 1 to 5"),
        _ => println!("others"),
    }
}


pub fn call5() {
    let x = 'c';

    match x {
        'a' ..= 'j' => println!("the beginning of ASCII letters"),
        'k' ..= 'z' => println!("the end of ASCII letters"),
        _ => println!("others"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn call6() {
    let p = Point {x: 0, y: 7};
    let Point {x: a, y: b} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);
}

pub fn call7() {
    let p = Point {x: 0, y: 7};
    let Point {x, y} = p;

    assert_eq!(0, x);
    assert_eq!(7, y);
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

pub fn call8() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("ChangeColor: R={},G={},B={}", r,g,b);
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("ChangeColor: H={},S={},V={}", h,s,v);
        },
        _ => {}
    }
}
