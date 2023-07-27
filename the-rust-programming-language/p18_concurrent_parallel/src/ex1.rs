use std::thread; // 모듈 경로를 현재 범위 안으로 가져오기
use std::time::Duration; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() -> Result<(), std::io::Error> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("새 스레드: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("주 스레드: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
