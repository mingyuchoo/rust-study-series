use std::sync::mpsc; // 모듈 경로를 현재 범위 안으로 가져오기
use std::thread; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() -> Result<(), std::io::Error> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("수신: {received}");

    Ok(())
}
