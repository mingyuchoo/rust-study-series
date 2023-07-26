use std::sync::mpsc; // 모듈 경로를 현재 범위 안으로 가져오기
use std::thread; // 모듈 경로를 현재 범위 안으로 가져오기
use std::time::Duration; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() -> Result<(), std::io::Error> {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("자식 스레드가"),
            String::from("안녕하세요"),
            String::from("라고"),
            String::from("인사합니다"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("그리고"),
            String::from("더 많은"),
            String::from("메시지를"),
            String::from("보냅니다"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("수신: {received}");
    }

    Ok(())
}
