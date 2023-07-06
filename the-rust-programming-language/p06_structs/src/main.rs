mod example; // 모듈 경로를 현재 범위 안으로 가져오기
mod instances; // 모듈 경로를 현재 범위 안으로 가져오기
mod methods; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    instances::create_instance();

    example::program1();
    example::program2();
    example::program3();
    example::print_rectangle();

    methods::call1();
    methods::call2();
    methods::call3();
}
