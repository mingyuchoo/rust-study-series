# react-app

Rust WASM 라이브러리를 React + TypeScript에서 사용하는 예제 프로젝트입니다.

## 기술 스택

- **런타임/패키지 매니저:** Bun
- **프레임워크:** React 19
- **빌드 도구:** Vite
- **언어:** TypeScript
- **테스트:** Vitest + Testing Library
- **WASM:** `wasm-lib` (로컬 Rust 크레이트)

## 사전 준비

- [Bun](https://bun.sh/) 설치
- `../wasm-lib` 크레이트를 `wasm-pack build`로 미리 빌드

## 의존성 설치

```bash
bun install
```

## 개발 서버 실행

```bash
bun run dev
```

## 프로덕션 빌드

```bash
bun run build
```

## 빌드 결과 미리보기

```bash
bun run preview
```

## 테스트

```bash
bun run test        # watch 모드
bun run test:run    # 단일 실행
```
