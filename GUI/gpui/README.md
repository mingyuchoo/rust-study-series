# gpui

Zed 에디터에서 사용하는 고성능 GPU 가속 UI 프레임워크입니다.

## gpui 프로젝트 생성

```bash
cargo binstall create-gpui-app
create-gpui-app --workspace --name my-gpui-app
cd my-gpui-app
```

## 빌드 및 실행

```bash
cargo build
# 또는
cargo build --release

cargo run
```

## 하위 프로젝트

- [hello-world](./hello-world/) - 기본 gpui 예제 애플리케이션
- [window-app](./window-app/) - 윈도우 기반 gpui 애플리케이션

## 참고 자료

- https://crates.io/crates/create-gpui-app
- https://www.gpui.rs/
