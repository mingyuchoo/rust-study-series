use std::error::Error; // 모듈 경로를 현재 범위 안으로 가져오기
use std::fs::File; // 모듈 경로를 현재 범위 안으로 가져오기
use std::io; // 모듈 경로를 현재 범위 안으로 가져오기
use std::io::Read; // 모듈 경로를 현재 범위 안으로 가져오기

pub fn call1() {
    let f1 = File::open("Hello.txt").unwrap();
    let f2 = File::open("Hello.txt").expect("파일을 열 수 없습니다.");
}

pub fn call2() {
    let f1 = read_username_from_file_1();

    fn read_username_from_file_1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // fn read_username_from_file_2() -> Result<String, io:Error> {
    //   let mut f = File::open("hello.txt")?;
    //   let mut s = String::new();
    //   f.read_to_string(&mut s)?;
    //   Ok(s)
    // }

    // fn read_username_from_file_3() -> Result<String, io:Error> {
    //   let mut s = String::new();
    //   File::open("hello.txt")?.read_to_string(&mut s)?;
    //   Ok(s)
    // }

    // fn read_username_from_file_4() -> Result<String, io:Error> {
    //   fs::read_to_string("hello.txt")?;
    // }
}

pub fn call3() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
