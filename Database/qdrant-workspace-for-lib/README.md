# qdrant-workspace-for-lib

러스트 Cargo 워크스페이스의 최소 예제를 다룹니다:

- 워크스페이스 생성 방법
- 라이브러리 크레이트 추가
- 라이브러리 예제 추가
- 워크스페이스 루트 또는 크레이트 디렉터리에서의 빌드 및 실행

이 워크스페이스에는 `create-collection`이라는 라이브러리 크레이트와 `basic-usage` 예제가 포함되어 있습니다.

## 사전 준비물

- 러스트 툴체인 설치 필요 (rustup, cargo, rustc)
- 이 워크스페이스는 2024 에디션을 사용합니다(워크스페이스 루트의 `Cargo.toml` 참조)

```bash
cargo install cargo-binstall
cargo install cargo-dist
cargo install cargo-edit
cargo install cargo-make
cargo install cargo-watch
cargo install cargo-udeps
cargo install cargo-tree
```

## 리포지토리 구조

```text
.
├─ Cargo.toml             # 워크스페이스 루트
├─ create-collection/
│  ├─ Cargo.toml
│  ├─ src/
│  │  └─ lib.rs
│  └─ examples/
│     └─ basic-usage.rs
```

## 워크스페이스 구성

워크스페이스 루트 Cargo.toml

- 멤버 선언: `create-collection`
- 워크스페이스 수준 패키지 메타데이터 설정
- 멤버 크레이트에 대한 경로 의존성 선택적 노출
- 루트에서 단일 `cargo` 명령으로 모든 크레이트를 빌드 가능

## 라이브러리 크레이트

크레이트: create-collection

`src/lib.rs` 현재 단순 함수만 노출합니다:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

`Cargo.toml` 은 라이브러리 확장을 위한 Qdrant 클라이언트 사용을 지원하기 위해 `qdrant-client`(serde 기능)를 의존성으로 선언합니다:

```toml
[dependencies]
qdrant-client = { version = "1.15.0", features = ["serde"] }
```

> 참고: 필요에 따라 `src/lib.rs`에 Qdrant 기능을 통합해 나갈 수 있습니다.

## 예제: basic-usage

- 파일: `create-collection/examples/basic-usage.rs`

라이브러리의 `add` 함수를 사용하는 간단 예제입니다:

```rust
use create_collection::add;

fn main() {
    let a: u64 = 2;
    let b: u64 = 3;
    let sum = add(a, b);
    println!("add({}, {}) = {}", a, b, sum);
}
```

> 중요: 러스트에서 크레이트 이름의 하이픈(`-`)은 코드에서는 언더스코어(`_`)로 변환됩니다. 따라서 패키지 `create-collection`은 코드에서 `create_collection`로 임포트합니다.

## 빌드

워크스페이스 루트 혹은 각 크레이트 디렉터리에서 빌드할 수 있습니다.

- 워크스페이스 전체 빌드(루트):

```powershell
cargo build
```

- 특정 크레이트만 빌드(`create-collection`):

```powershell
cargo build --package create-collection
```

## 예제 실행

- 워크스페이스 루트에서 실행:

```powershell
cargo run --package create-collection --example basic-usage
```

- `create-collection/` 내부에서 실행:

```powershell
cargo run --example basic-usage
```

예상 출력:

```text
add(2, 3) = 5
```

## 테스트

라이브러리에는 `add`에 대한 기본 단위 테스트가 포함되어 있습니다.

- 워크스페이스 루트에서:

```powershell
cargo test
```

- 혹은 `create-collection` 크레이트만 테스트:

```powershell
cargo test --package create-collection
```

## 이 워크스페이스를 처음부터 만들기(참고)

아래 단계를 통해 동일한 구성을 재현할 수 있습니다.

새 디렉터리를 만들고 워크스페이스를 초기화합니다:

```powershell
mkdir qdrant-workspace-for-lib
cd qdrant-workspace-for-lib
cargo new --vcs none --name qdrant-workspace-for-lib --bin temp-bin
```

루트 `Cargo.toml`을 다음 내용으로 교체합니다:

```toml
[workspace]
  members  = ["create-collection"]
  resolver = "2"

  [workspace.dependencies]
    create-collection = { path = "./create-collection" }

  [workspace.package]
    authors = ["Mingyu Choo <mingyuchoo@gmail.com>"]
    edition = "2024"
    version = "0.1.0"
```

임시 바이너리 크레이트 파일 `src/main.rs`를 제거하고 필요에 맞게 조정합니다.

멤버 라이브러리 크레이트를 추가합니다:

```powershell
cargo new create-collection --lib
```

`create-collection/Cargo.toml`에 의존성과 예제 선언을 추가합니다:

```toml
[package]
  authors.workspace = true
  edition.workspace = true
  name              = "create-collection"
  version.workspace = true

[dependencies]
  qdrant-client = { version = "1.15.0", features = ["serde"] }

[[example]]
name = "basic-usage"
path = "examples/basic-usage.rs"
```

위와 같이 `src/lib.rs`와 `examples/basic-usage.rs`를 추가합니다.

빌드 및 실행:

```powershell
cargo build
cargo run --package create-collection --example basic-usage
```

## 참고

- <https://github.com/qdrant/rust-client>
