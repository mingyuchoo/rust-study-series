mod at_binding; // 모듈 경로를 현재 범위 안으로 가져오기
mod for_loop; // 모듈 경로를 현재 범위 안으로 가져오기
mod func_param; // 모듈 경로를 현재 범위 안으로 가져오기
mod if_let; // 모듈 경로를 현재 범위 안으로 가져오기
mod ignore_pattern_value; // 모듈 경로를 현재 범위 안으로 가져오기
mod match_guard; // 모듈 경로를 현재 범위 안으로 가져오기
mod pattern_grammer; // 모듈 경로를 현재 범위 안으로 가져오기
mod while_let; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    if_let::call();
    while_let::call();
    for_loop::call();
    func_param::call();
    pattern_grammer::call1();
    pattern_grammer::call2();
    pattern_grammer::call3();
    pattern_grammer::call4();
    pattern_grammer::call5();
    pattern_grammer::call6();
    pattern_grammer::call7();
    pattern_grammer::call8();

    ignore_pattern_value::call1();
    ignore_pattern_value::call2();
    ignore_pattern_value::call3();
    ignore_pattern_value::call4();
    ignore_pattern_value::call5();
    ignore_pattern_value::call6();
    ignore_pattern_value::call7();

    match_guard::call1();
    match_guard::call2();
    match_guard::call3();

    at_binding::call1();
}
