mod ex1; // 모듈 경로를 현재 범위 안으로 가져오기
mod ex2; // 모듈 경로를 현재 범위 안으로 가져오기
mod ex3; // 모듈 경로를 현재 범위 안으로 가져오기
mod ex4; // 모듈 경로를 현재 범위 안으로 가져오기
mod ex5; // 모듈 경로를 현재 범위 안으로 가져오기
mod ex6; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() -> Result<(), std::io::Error> {
    ex1::call1().expect("panicked!");
    ex2::call1().expect("panicked!");
    ex3::call1().expect("panicked!");
    ex4::call1().expect("panicked!");
    ex5::call1().expect("panicked!");
    ex6::call1()
}
