# `my-rust-addon`

![CI](https://github.com/napi-rs/package-template/workflows/CI/badge.svg)

> [napi-rs](https://napi.rs) v3을 사용하여 Rust로 Node.js 네이티브 애드온을 작성하는 프로젝트입니다.

## 주요 기능

현재 구현된 네이티브 함수:

- `plus100(input: number): number` - 입력값에 100을 더하여 반환

## 주요 의존성

| 패키지 | 버전 | 용도 |
|--------|------|------|
| napi | 3.0.0 | Node.js N-API 바인딩 |
| napi-derive | 3.0.0 | `#[napi]` 매크로 |
| napi-build | 2 | 빌드 스크립트 |
| @napi-rs/cli | ^3.2.0 | NAPI-RS CLI 도구 |
| ava | ^7.0.0 | 테스트 프레임워크 |
| oxlint | ^1.14.0 | 린터 |

## 요구 사항

- [Rust](https://www.rust-lang.org/tools/install) (최신 stable 버전)
- [Node.js](https://nodejs.org) v12.22+ / v14.17+ / v15.12+ / v16+
- [Bun](https://bun.sh) v1.2 이상

## 로컬 개발 환경 설정

```bash
# 의존성 설치
bun install

# 네이티브 애드온 빌드 (release 모드)
bun run build

# 테스트 실행
bun run test
```

테스트 실행 결과 예시:

```bash
$ ava --verbose

  ✔ sync function from native code
  ─

  1 test passed
```

## 주요 기능

### 빌드

`bun run build` 명령을 실행하면 프로젝트 루트에 `my-rust-addon.[darwin|win32|linux].node` 파일이 생성됩니다.
이 파일은 [src/lib.rs](./src/lib.rs)에서 빌드된 네이티브 애드온입니다.

디버그 모드로 빌드하려면:

```bash
bun run build:debug
```

### 테스트

[ava](https://github.com/avajs/ava)를 사용하여 네이티브 애드온을 테스트합니다.

```bash
bun run test
```

### 벤치마크

```bash
bun run bench
```

### 코드 포맷 및 린트

```bash
# 전체 포맷 (Prettier + Rust + TOML)
bun run format

# 린트
bun run lint
```

### CI/CD

GitHub Actions를 통해 커밋 및 풀 리퀘스트마다 자동으로 빌드 및 테스트가 실행됩니다.

지원 플랫폼: `x86_64-pc-windows-msvc`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`

### 릴리스

패키지 릴리스는 GitHub Actions를 통해 자동화되어 있습니다.

각 플랫폼별로 별도의 npm 패키지로 배포되며, `optionalDependencies`에 추가됩니다.
NPM이 자동으로 현재 플랫폼에 맞는 네이티브 패키지를 선택하여 설치합니다.

릴리스 방법:

```bash
npm version [major | minor | patch]

git push
```

GitHub Actions가 나머지 작업(빌드, 테스트, 배포)을 자동으로 처리합니다.

> 주의: `npm publish`를 직접 실행하지 마세요.

릴리스를 위해 GitHub 프로젝트 설정의 `Settings -> Secrets`에 **NPM_TOKEN**을 등록해야 합니다.
