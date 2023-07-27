use std::thread; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() -> Result<(), std::io::Error> {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("벡터: {v:?}");
    });

    // drop(v); // ERROR

    handle.join().unwrap();

    Ok(())
}
