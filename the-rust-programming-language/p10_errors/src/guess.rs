struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "유추한 값은 반드시 1에서 100 사이의 값이어야 합니다. 입력한 값: {}",
                value
            );
        }

        // return new Guess instance
        Guess { value } // DO NOT add ';' for return value
    }

    fn value(&self) -> i32 {
        self.value
    }
}

pub fn call1() {
    let g = Guess::new(100);
    println!("Guess: {}", g.value());
}
