mod hash_maps; // 모듈 경로를 현재 범위 안으로 가져오기
mod strings; // 모듈 경로를 현재 범위 안으로 가져오기
mod vectors; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    vectors::call1();

    strings::call1();
    strings::call2();
    strings::call3();
    strings::call4();
    strings::call5();
    strings::call6();

    hash_maps::call1();
    hash_maps::call2();
    hash_maps::call3();
    hash_maps::call4();
    hash_maps::call5();
}
