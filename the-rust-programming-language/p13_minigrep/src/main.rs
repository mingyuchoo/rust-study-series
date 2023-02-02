use p13_minigrep::run;
use p13_minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("입력한 인수를 구문분석하는 동안 오류가 발생했습니다. 확인해보니 {err}");
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("애플리케이션 에러: {e}");
        process::exit(1);
    }
}
