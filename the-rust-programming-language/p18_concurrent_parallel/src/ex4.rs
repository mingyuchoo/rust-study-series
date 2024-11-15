use std::sync::mpsc;
use std::thread;

pub fn call1() -> Result<(), std::io::Error> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val)
          .unwrap();
    });

    let received = rx.recv()
                     .unwrap();

    println!("수신: {received}");

    Ok(())
}
