pub fn call1() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3, 4, 5];
    for i in &v2 {
        println!("{}", i);
    }

    let third: &i32 = &v2[2];
    println!("세 번째 원소: {}", third);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    for i in &mut v3 {
        *i += 50;
    }

    for i in &mut v3 {
        println!("{}", i);
    }
}

pub fn call2() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
    ];
}
