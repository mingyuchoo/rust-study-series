use add_one; // 모듈 경로를 현재 범위 안으로 가져오기
use art::mix; // 모듈 경로를 현재 범위 안으로 가져오기
use art::PrimaryColor; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    let num = 10;
    println!(
        "안녕하세요? {} 더하기 1은 {}입니다!",
        num,
        add_one::add_one(num)
    );

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
