pub fn call1() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

pub fn call2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("match, n={:?}", n),
        _ => println!("not match, x = {:?}", x),
    }
    println!("result: x={:?}, y={:?}", x, y);
}

pub fn call3() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }
}
