mod boxes; // 모듈 경로를 현재 범위 안으로 가져오기
mod derefs; // 모듈 경로를 현재 범위 안으로 가져오기
mod drops; // 모듈 경로를 현재 범위 안으로 가져오기
mod memory_leak; // 모듈 경로를 현재 범위 안으로 가져오기
mod rcs; // 모듈 경로를 현재 범위 안으로 가져오기
mod refcells; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    boxes::call1();
    boxes::call2();

    derefs::call3();

    drops::call1();
    drops::call2();

    rcs::call1();

    refcells::call1();

    memory_leak::call1();
    memory_leak::call2();
}
