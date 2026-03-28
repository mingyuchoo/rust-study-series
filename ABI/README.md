# Rust로 Node.js 네이티브 모듈 만들기

Node.js에서 Rust 코드를 사용하는 방법은 주로 **NAPI-RS** 라이브러리를 활용합니다. Rust로 컴파일된 `.node` 바이너리 파일을 npm 패키지처럼 배포/설치할 수 있습니다.

---

## 전체 흐름

```
Rust 코드 작성 → cargo build → .node 바이너리 생성 → npm 패키지로 배포 → npm install로 설치
```

---

## 하위 프로젝트

- [my-rust-addon](./my-rust-addon/) - NAPI-RS v3 기반 네이티브 애드온 예제

## 1. 프로젝트 초기화

```bash
# NAPI-RS CLI 설치
npm install -g @napi-rs/cli

# 새 프로젝트 생성
napi new my-rust-addon
cd my-rust-addon
```

실제 `my-rust-addon` 프로젝트 구조:
```
my-rust-addon/
├── Cargo.toml
├── package.json
├── build.rs
├── src/
│   └── lib.rs         <- Rust 코드 작성
├── __test__/
│   └── index.spec.ts  <- 테스트
├── benchmark/
├── index.js           <- JS 진입점
├── index.d.ts         <- TypeScript 타입 정의
└── bun.lock
```

---

## 2. Rust 코드 작성 (`src/lib.rs`)

현재 `my-rust-addon`에 구현된 실제 코드:

```rust
#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
```

아래는 확장 예시 (덧셈, 문자열, 피보나치 등):

```rust
use napi_derive::napi;

#[napi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub fn greet(name: String) -> String {
    format!("안녕하세요, {}님!", name)
}

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

실제 `my-rust-addon/Cargo.toml`:

```toml
[package]
name    = "my_rust_addon"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Node.js가 로드할 수 있는 동적 라이브러리

[dependencies]
napi        = "3.0.0"
napi-derive = "3.0.0"

[build-dependencies]
napi-build = "2"

[profile.release]
lto   = true
strip = "symbols"
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

실제 프로젝트에서는 **Bun** (v1.2+)을 패키지 매니저로 사용하며, `@napi-rs/cli` v3을 사용합니다.
주요 스크립트:

```json
{
  "scripts": {
    "build": "napi build --platform --release",
    "build:debug": "napi build --platform",
    "test": "ava",
    "bench": "bun benchmark/bench.ts",
    "format": "run-p format:prettier format:rs format:toml",
    "lint": "oxlint ."
  },
  "devDependencies": {
    "@napi-rs/cli": "^3.2.0",
    "ava": "^7.0.0",
    "oxlint": "^1.14.0"
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
# 의존성 설치
bun install

# 빌드 (Rust -> .node 바이너리)
bun run build

# 테스트 (ava 사용)
bun run test

# 벤치마크
bun run bench
```

현재 테스트 예시 (`__test__/index.spec.ts`):

```typescript
import { plus100 } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})
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
