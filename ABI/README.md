# Rust로 Node.js 네이티브 모듈 만들기

Node.js에서 Rust 코드를 사용하는 방법은 주로 **NAPI-RS** 라이브러리를 활용합니다. Rust로 컴파일된 `.node` 바이너리 파일을 npm 패키지처럼 배포/설치할 수 있습니다.

---

## 전체 흐름

```
Rust 코드 작성 → cargo build → .node 바이너리 생성 → npm 패키지로 배포 → npm install로 설치
```

---

## 1. 프로젝트 초기화

```bash
# NAPI-RS CLI 설치
npm install -g @napi-rs/cli

# 새 프로젝트 생성
napi new my-rust-addon
cd my-rust-addon
```

생성되는 구조:
```
my-rust-addon/
├── Cargo.toml
├── package.json
├── src/
│   └── lib.rs       ← Rust 코드 작성
└── index.js         ← JS 진입점
```

---

## 2. Rust 코드 작성 (`src/lib.rs`)

```rust
use napi_derive::napi;

// 단순 덧셈 함수
#[napi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 문자열 처리 함수
#[napi]
pub fn greet(name: String) -> String {
    format!("안녕하세요, {}님!", name)
}

// 무거운 계산 (Rust의 성능이 빛나는 경우)
#[napi]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            b
        }
    }
}
```

---

## 3. `Cargo.toml` 설정

```toml
[package]
name = "my-rust-addon"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Node.js가 로드할 수 있는 동적 라이브러리

[dependencies]
napi = { version = "2", features = ["napi4"] }
napi-derive = "2"

[build-dependencies]
napi-build = "2"
```

---

## 4. `build.rs` (빌드 스크립트)

```rust
extern crate napi_build;

fn main() {
    napi_build::setup();
}
```

---

## 5. `package.json` 설정

```json
{
  "name": "my-rust-addon",
  "version": "1.0.0",
  "main": "index.js",
  "scripts": {
    "build": "napi build --platform --release",
    "prepublishOnly": "napi build --platform --release"
  },
  "napi": {
    "name": "my-rust-addon",
    "triples": {
      "defaults": true
    }
  },
  "devDependencies": {
    "@napi-rs/cli": "^2.0.0"
  }
}
```

---

## 6. `index.js` (JS 진입점)

```javascript
// NAPI-RS가 자동 생성하지만, 수동으로 작성 시:
const { add, greet, fibonacci } = require('./my-rust-addon.node')

module.exports = { add, greet, fibonacci }
```

---

## 7. 빌드 및 테스트

```bash
# 빌드 (Rust → .node 바이너리)
npm run build

# 테스트
node -e "
const addon = require('.');
console.log(addon.add(1, 2));          // 3
console.log(addon.greet('철수'));       // 안녕하세요, 철수님!
console.log(addon.fibonacci(40));      // 102334155
"
```

---

## 8. npm 배포 및 설치

```bash
# npm에 배포
npm publish

# 다른 프로젝트에서 설치
npm install my-rust-addon
```

```javascript
// 다른 프로젝트에서 사용
const { fibonacci } = require('my-rust-addon')
console.log(fibonacci(50)) // 매우 빠름!
```

---

## 크로스 플랫폼 배포 전략

실제 배포 시엔 OS별로 다른 바이너리가 필요합니다. NAPI-RS는 이를 **optionalDependencies**로 처리합니다:

```json
{
  "optionalDependencies": {
    "my-rust-addon-darwin-x64": "1.0.0",
    "my-rust-addon-linux-x64-gnu": "1.0.0",
    "my-rust-addon-win32-x64-msvc": "1.0.0"
  }
}
```

`npm install` 시 현재 OS에 맞는 바이너리만 자동으로 선택됩니다. 이 방식은 **@swc/core**, **@napi-rs/magic-string** 등 유명 패키지들이 실제로 사용하는 방식입니다.

---

## 언제 Rust 네이티브 모듈을 쓰나?

| 상황 | 추천 |
|------|------|
| CPU 집약적 연산 (이미지 처리, 암호화) | ✅ Rust |
| 메모리 안전성이 중요한 파싱 | ✅ Rust |
| 단순 I/O, 비동기 작업 | ❌ 순수 JS로 충분 |
| 빠른 프로토타이핑 | ❌ 빌드 복잡도가 있음 |
