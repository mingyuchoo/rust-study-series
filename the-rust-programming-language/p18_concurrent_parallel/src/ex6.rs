use std::sync::{Arc, Mutex}; // 모듈 경로를 현재 범위 안으로 가져오기
use std::thread; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() -> Result<(), std::io::Error> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap());

    Ok(())
}
