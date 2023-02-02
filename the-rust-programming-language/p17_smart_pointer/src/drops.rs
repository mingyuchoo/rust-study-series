struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다!", self.data);
    }
}

pub fn call1() {
    let c = CustomSmartPointer {
        data: String::from("내 데이터"),
    };
    let d = CustomSmartPointer {
        data: String::from("남 데이터"),
    };
    println!("CustomSmartPointer를 생성했습니다.");
}

pub fn call2() {
    let c = CustomSmartPointer {
        data: String::from("내 데이터"),
    };
    println!("CustomSmartPointer를 생성했습니다.");
    drop(c);
    println!("CustomSmartPoint를 call2 함수 끝에 도달하기 전에 해제합니다.");
}
