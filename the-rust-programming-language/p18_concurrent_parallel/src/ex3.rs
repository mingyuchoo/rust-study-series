use std::thread;

pub fn call1() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("벡터: {v:?}");
    });

    // drop(v); // ERROR

    handle.join().unwrap();
}
