# qdrant-workspace-for-lib

러스트 Cargo 워크스페이스의 최소 예제를 다룹니다:

- 워크스페이스 생성 방법
- 라이브러리 크레이트 추가
- 라이브러리 예제 추가
- 워크스페이스 루트 또는 크레이트 디렉터리에서의 빌드 및 실행
- .env 기반 설정으로 하드코딩 제거 (dotenvy)

이 워크스페이스에는 `create-collection`, `delete-collection` 두 개의 라이브러리 크레이트와 각 크레이트의 `basic-usage` 예제가 포함되어 있습니다. 예제는 `.env`로부터 설정을 읽어 실행됩니다.

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
├─ Cargo.toml                  # 워크스페이스 루트
├─ .env.example                # 환경 변수 예시 파일 (.env로 복사하여 사용)
├─ create-collection/
│  ├─ Cargo.toml
│  ├─ src/
│  │  └─ lib.rs
│  └─ examples/
│     └─ basic-usage.rs
└─ delete-collection/
   ├─ Cargo.toml
   ├─ src/
   │  └─ lib.rs
   └─ examples/
      └─ basic-usage.rs
```

## 환경 변수 설정 (.env)

예제는 런타임에 `.env` 파일에서 설정을 읽습니다. 루트의 `.env.example`를 복사하여 `.env`를 생성하고 값을 채우세요.

```powershell
Copy-Item .env.example .env
```

필요한 키:

- `QDRANT_URL` (예: `http://localhost:6334`)
- `QDRANT_COLLECTION_NAME_1` (예: `my_collection`)
- `QDRANT_COLLECTION_NAME_2` (예: `custom_collection`)
- `QDRANT_VECTOR_SIZE` (선택, 기본값 384)

두 예제 모두 시작 시 `dotenvy`로 `.env`를 로드하고, `std::env::var`로 환경 변수를 읽습니다.

## 워크스페이스 구성

워크스페이스 루트 Cargo.toml

- 멤버 선언: `create-collection`, `delete-collection`
- 워크스페이스 수준 패키지 메타데이터 설정
- 멤버 크레이트에 대한 경로 의존성 선택적 노출
- 루트에서 단일 `cargo` 명령으로 모든 크레이트를 빌드 가능

## 라이브러리 크레이트

크레이트: create-collection

Qdrant 클라이언트를 사용하여 컬렉션을 생성하는 API를 제공합니다.

`Cargo.toml` 은 `qdrant-client`(serde 기능) 및 예제 실행용 비동기 런타임 `tokio`, 환경 변수 로딩을 위한 `dotenvy`를 의존성으로 선언합니다:

```powershell
cargo add --package create-collection qdrant-client --features serde
cargo add --package create-collection tokio --features full --vers 1.0
cargo add --package create-collection dotenvy --vers 0.15
```

`create-collection` 패키지에 아래와 같이 의존성이 추가됩니다.

```toml
[dependencies]
qdrant-client = { version = "1.15.0", features = ["serde"] }
tokio = { version = "1.0", features = ["full"] }
dotenvy = "0.15"
```

> 참고: 필요에 따라 `src/lib.rs`에 Qdrant 기능을 통합해 나갈 수 있습니다.

크레이트: delete-collection

Qdrant 컬렉션을 삭제하는 API를 제공합니다. 의존성 구성은 `create-collection`과 유사하며, 예제는 동일한 `.env`를 사용합니다.

## 예제: basic-usage

- 파일: `create-collection/examples/basic-usage.rs`
- 파일: `delete-collection/examples/basic-usage.rs`

환경 변수를 사용하여 Qdrant에 접속하고, 컬렉션 생성/삭제를 수행합니다.

예제 실행 전 `.env`를 준비해야 합니다.

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

- 특정 크레이트만 빌드(`delete-collection`):

```powershell
cargo build --package delete-collection
```

## 예제 실행

- 워크스페이스 루트에서 실행:

```powershell
# 컬렉션 생성 예제
cargo run --package create-collection --example basic-usage

# 컬렉션 삭제 예제
cargo run --package delete-collection --example basic-usage
```

- `create-collection/` 내부에서 실행:

```powershell
cargo run --example basic-usage
```

- `delete-collection/` 내부에서 실행:

```powershell
cargo run --example basic-usage
```

예상 동작:

- 생성 예제: `.env`의 컬렉션 이름들로 컬렉션이 생성됩니다.
- 삭제 예제: 동일한 컬렉션 이름들로 컬렉션이 삭제됩니다.

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
  members  = ["create-collection", "delete-collection"]
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
cargo new delete-collection --lib
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
  tokio         = { version = "1.0", features = ["full"] }
  dotenvy       = "0.15"

[[example]]
name = "basic-usage"
path = "examples/basic-usage.rs"
```

`delete-collection/Cargo.toml`에도 유사하게 의존성과 예제 선언을 추가합니다:

```toml
[package]
  authors.workspace = true
  edition.workspace = true
  name              = "delete-collection"
  version.workspace = true

[dependencies]
  qdrant-client = { version = "1.15.0", features = ["serde"] }
  tokio         = { version = "1.47.1", features = ["full"] }
  dotenvy       = "0.15"

[[example]]
  name = "basic-usage"
  path = "examples/basic-usage.rs"
```

루트에 `.env.example`를 추가하고, `.env`로 복사하여 값을 채운 뒤 예제를 실행합니다.

빌드 및 실행:

```powershell
cargo build --workspace --examples
cargo run --package create-collection --example basic-usage
cargo run --package delete-collection --example basic-usage
```

## 참고

- <https://github.com/qdrant/rust-client>
- <https://www.geeksforgeeks.org/data-science/qdrant/>
