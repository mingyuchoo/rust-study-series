# Leptos-CSR

Leptos 프레임워크 CSR(클라이언트 사이드 렌더링) 튜토리얼 프로젝트입니다.
[Trunk](https://trunkrs.dev/) 빌드 도구를 사용하며, Leptos 공식 문서의 예제를 단계별로 학습할 수 있습니다.

## 프로젝트 구조

```
leptos-csr/
├── src/
│   ├── main.rs              # 진입점
│   ├── lib.rs               # App 컴포넌트 및 라우팅 정의
│   ├── part1/               # Part 1 튜토리얼
│   │   ├── basic_component.rs
│   │   ├── dynamic_attributes.rs
│   │   ├── components_and_props.rs
│   │   ├── iteration.rs
│   │   ├── iteration_with_for.rs
│   │   ├── forms_and_inputs.rs
│   │   ├── control_flow.rs
│   │   ├── error_handling.rs
│   │   ├── parent_child_communication.rs
│   │   ├── passing_children_to_components.rs
│   │   ├── reactivity/          # 반응성 관련
│   │   ├── asynchronous/        # 비동기 처리
│   │   ├── interlude/           # Children 프로젝션
│   │   ├── globalstatemanagement/  # 전역 상태 관리
│   │   ├── routing/             # 라우팅
│   │   ├── styling/             # Tailwind CSS 스타일링
│   │   └── testing/             # 테스트
│   └── part2/
│       └── typicode.rs          # 외부 API 호출 예제
├── index.html
├── Cargo.toml
├── Trunk.toml
├── tailwind.config.js
└── style/
```

## 사전 준비

```shell
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install leptosfmt
```

## 실행 방법

```shell
trunk serve --port 3000 --open
```

접속 주소: `http://localhost:3000`

## 주요 의존성

| 크레이트 | 버전 | 용도 |
|---------|------|------|
| `leptos` | 0.6.15 | 반응형 웹 프레임워크 (CSR, nightly) |
| `leptos_meta` | 0.6.15 | 메타 태그 관리 |
| `leptos_router` | 0.6.15 | 클라이언트 사이드 라우팅 |
| `gloo-net` | 0.6.0 | HTTP 클라이언트 (WASM용) |
| `gloo-timers` | 0.3.0 | 타이머 유틸리티 |
| `serde` / `serde_json` | 1.0.215 / 1.0.132 | 직렬화/역직렬화 |
| `uuid` | 1.11.0 | UUID v4 생성 |
| `web-sys` | 0.3.72 | Web API 바인딩 |

## 주요 기능 / 라우트

- `/` - 홈 (모든 예제 링크 목록)
- `/part1/basic_component/*` - 기본 컴포넌트
- `/part1/dynamic_attributes/*` - 동적 속성
- `/part1/components_and_props/*` - 컴포넌트와 Props
- `/part1/iteration/*` - 반복 렌더링
- `/part1/forms_and_inputs/*` - 폼과 입력 (Controlled/Uncontrolled)
- `/part1/control_flow/*` - 조건부 렌더링
- `/part1/error_handling/*` - 에러 핸들링
- `/part1/parent_child_communication/*` - 부모-자식 통신
- `/part1/reactivity/*` - 반응성 (시그널, 이펙트)
- `/part1/asynchronous/*` - 비동기 (Resource, Suspense, Action)
- `/part1/routing/*` - 라우팅
- `/part1/styling/*` - Tailwind CSS 스타일링
- `/part2/typicode/Api` - JSONPlaceholder API 호출 예제

## 참고 자료

- [Building User Interfaces](https://book.leptos.dev/view/index.html)
