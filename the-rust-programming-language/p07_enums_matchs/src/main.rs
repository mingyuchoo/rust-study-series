mod enumerations; // 모듈 경로를 현재 범위 안으로 가져오기
mod matches; // 모듈 경로를 현재 범위 안으로 가져오기
mod options; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    enumerations::call1();

    options::call1();
    options::call2();
    options::call3();

    matches::call1();
    matches::call2();
    matches::call3();
}
