pub fn call() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("인덱 {index}의 값: {value}");
    }
}
