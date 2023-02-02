pub fn call1() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{}", some_number.unwrap());
    println!("{}", some_string.unwrap());
}

pub fn call2() {
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    // The Match Control Flow
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("행운의 페니!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        } // NO semicolon here to return
    }

    println!("{:#?}", Coin::Penny);
    println!("{:#?}", value_in_cents(Coin::Penny));
    println!("{:#?}", value_in_cents(Coin::Nickel));
    println!("{:#?}", value_in_cents(Coin::Dime));
    println!("{:#?}", value_in_cents(Coin::Quarter));
}

pub fn call3() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            }
        } // NO semicolon here to return
    }
    println!("{:#?}", value_in_cents(Coin::Penny));
    println!("{:#?}", value_in_cents(Coin::Nickel));
    println!("{:#?}", value_in_cents(Coin::Dime));
    println!("{:#?}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let mut count = 0;
    let coin = Coin::Dime;

    if let Coin::Quarter(state) = coin {
        println!("{state:?}주의  25센트 동전!");
    } else {
        count += 1;
    }

    println!("count: {count}");
}
