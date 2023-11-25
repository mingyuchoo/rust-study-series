# README
## Why did I make this?

## For learn thow to build an application

- 명령줄 인수 처리하기
- 인수 값 읽어오기
- 인수 값을 변수에 저장하기
- 파일 읽기
- 모듈화하기
- 인수 구분분석 분리하기
- 설정값 그룹짓기
- 구조체 생성자 만들기
- 에러처리 하기
- 에러 메시지 개선하기
- panic! 매크로 디신 Result 사용하기
- 함수 호출하고 에러처리하기
- 기능 개발하기
- 로직분리하기
- 코드를 라이브러리로 떼내기
- 실패하는 테스트 작성하기
- 테스트가 성공하도록 코드 작성하기
- 환경 변수 처리하기
- 에러 메시지 출력하기
- 에러 기록 확인하기
- 에러를 stderr에 출력하기

## How do you use this?

```bash
rustup default stable
rustup update stable

cargo run test poem.txt
# or
./run.sh
```

## What do you get from this?

## How can you test this?

```bash
cargo test
```

## How to use watch mode

### Install `cargo-watch` for watch mode

```bash
cargo install cargo-watch
```

### Run as watch mode with `cargo-watch`

```bash
# Run test only
$ cargo watch -x test

# Run check then tests
$ cargo watch -x check -x test

# Run run current application
cargo watch -x 'run --bin app'

# Run run with arguments
$ cargo watch -x 'run -- --some-arg'

# run an arbitrary command
$ cargo watch -- echo Hello world

# Run with features passed to carg
$ cargo watch --features "foo,bar"
```
