use std::collections::HashMap; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() {
    let mut scores1 = HashMap::new();

    scores1.insert(String::from("블루"), 10);
    scores1.insert(String::from("옐로"), 50);

    for (key, value) in &scores1 {
        println!("{key}: {value}");
    }

    let teams = vec![String::from("블루"), String::from("옐로")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores2 {
        println!("{key}: {value}");
    }
}

pub fn call2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("블루");
    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    // field_name과 field_value 변수는 이 지점부터 유효하지 않다.
    // 이 값들을 사용하려고 하면 컴파일러가 에러를 발생한다.
}

pub fn call3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 10);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let team_name = String::from("블루");
    let score = scores.get(&team_name);

    println!("{team_name}: {score:?}");
}

pub fn call4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.entry(String::from("옐로")).or_insert(50);
    scores.entry(String::from("블루")).or_insert(50);

    println!("{scores:?}");
}

pub fn call5() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
