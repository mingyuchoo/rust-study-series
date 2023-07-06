mod generic_data_types; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기
mod generic_functions; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기
mod lifetime; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기
mod traits; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기

fn main() {
    generic_functions::call1();
    generic_functions::call2();

    generic_data_types::call1();
    generic_data_types::call3();
    generic_data_types::call4();
    generic_data_types::call5();
    generic_data_types::call6();

    traits::call1();
    traits::call2();
    traits::call3();
    traits::call4();
    traits::call5();
    traits::call6();
    traits::call7();

    lifetime::call1();
    lifetime::call2();
    lifetime::call3();
    lifetime::call4();
}
