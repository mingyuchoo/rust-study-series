pub fn call1() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("id를 범위에서 찾았습니: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("id를 다른 범위에서 찾았습니다.")
        }
        Message::Hello { id } => {
            println!("다른 id {}를 찾았습다.", id)
        }
    }
}
