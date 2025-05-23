use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("숫자를 맞혀봅시다!");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("게임 플레이어가 맞혀야 할 숫자: {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            | Ok(num) => num,
            | Err(_) => continue,
        };

        println!("입력한 값: {}", guess);

        // match expression
        match guess.cmp(&secret_number) {
            | Ordering::Less => println!("입력한 숫자가 작습니다!"),
            | Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            | Ordering::Equal => {
                println!("정답!");
                break;
            },
        }
    }

    Ok(())
}
