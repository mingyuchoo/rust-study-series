/// if conditions
pub fn if_conditions() {
    let number = 3;

    if number < 5 {
        println!("조건이 일치합니다.");
    } else {
        println!("조건이 일치하지 않습니다.");
    }

    if number != 0 {
        println!("변수에 저장된 값이 0이 아닙니다.");
    }

    let new_number = 6;
    if new_number % 4 == 0 {
        println!("new_number is divisible by 4");
    } else if new_number % 3 == 0 {
        println!("new_number is divisible by 3");
    } else if new_number % 2 == 0 {
        println!("new_number is divisible by 2");
    } else {
        println!("new_number is NOT divisible by 4, 3, or 2");
    }
}

/// Using if in a let statement
pub fn let_and_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number의 값: {number}");
}

/// Returning valus from loops
pub fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result의 값: {result}");
}

/// Loop labels to disambiguate between multiple loops
pub fn labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

/// Conditional loops with while
pub fn whiles() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("발사!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

/// for element
pub fn for_element() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {element}");
    }
}

/// for reverse
pub fn for_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}
