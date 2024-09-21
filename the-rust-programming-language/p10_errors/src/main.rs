mod guess; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기
mod recoverable; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기
mod unrecoverable; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unrecoverable::call1();

    recoverable::call1();
    recoverable::call2();
    recoverable::call3();

    guess::call1();

    Ok(())
}
