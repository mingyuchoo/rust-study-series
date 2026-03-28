# p15_minigrep_refined

> The Rust Programming Language - minigrep 개선 버전

## 프로젝트를 만든 이유

## 애플리케이션 빌드 방법 학습 (개선 버전)

1. 명령줄 인수 처리하기
   1. 인수 값 읽어오기
   2. 인수 값을 변수에 저장하기
2. 파일 읽기
3. 모듈화하기
   1. 인수 구분분석 분리하기
   2. 설정값 그룹짓기
   3. 구조체 생성자 만들기
4. 에러처리 하기
   1. 에러 메시지 개선하기
   2. panic! 매크로 대신 Result 사용하기
   3. 함수 호출하고 에러처리하기
5. 기능 개발하기
   1. 로직분리하기
   2. 코드를 라이브러리로 떼내기
   3. 실패하는 테스트 작성하기
   4. 테스트가 성공하도록 코드 작성하기
6. 환경 변수 처리하기
7. 에러 메시지 출력하기
   1. 에러 기록 확인하기
   2. 에러를 stderr에 출력하기

## 프로젝트 구조

```
p15_minigrep_refined/
  Cargo.toml
  src/
    main.rs    # 진입점
    lib.rs     # 라이브러리 (검색 로직)
```

## 주요 의존성

- Rust Edition: 2024
- 외부 의존성 없음

## 실행 방법

```bash
rustup default stable
rustup update stable

cargo run test poem.txt
# 또는
./run.sh
```

## 테스트 방법

```bash
cargo test
```

## Watch 모드 사용법

### `cargo-watch` 설치

```bash
cargo install cargo-watch
```

### `cargo-watch`로 Watch 모드 실행

```bash
# 테스트만 실행
cargo watch -x test

# check 후 테스트 실행
cargo watch -x check -x test

# 현재 애플리케이션 실행
cargo watch -x 'run --bin app'

# 인수를 전달하여 실행
cargo watch -x 'run -- --some-arg'

# 임의의 명령어 실행
cargo watch -- echo Hello world

# feature를 전달하여 실행
cargo watch --features "foo,bar"
```
