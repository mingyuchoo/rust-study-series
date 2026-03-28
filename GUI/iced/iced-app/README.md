# Iced 주소록 앱

Iced GUI 프레임워크와 Clean Architecture를 적용한 주소록 데스크톱 애플리케이션입니다. SQLite를 사용하여 주소 데이터를 관리합니다.

## 주요 기능

- Clean Architecture 기반 계층 분리 (domain, application, infrastructure, presentation)
- SQLite 데이터베이스를 이용한 주소 데이터 영구 저장
- Iced 프레임워크를 활용한 GUI 인터페이스

## 프로젝트 구조

```
iced-app/
├── Cargo.toml          # Workspace 루트 설정
├── domain/             # 도메인 계층 (엔티티, 리포지토리 트레이트)
│   └── Cargo.toml      # 의존성: serde
├── application/        # 애플리케이션 계층 (유스케이스)
│   └── Cargo.toml      # 의존성: domain
├── infrastructure/     # 인프라 계층 (SQLite 구현체)
│   └── Cargo.toml      # 의존성: domain, rusqlite 0.32
├── presentation/       # 표현 계층 (Iced UI)
│   └── Cargo.toml      # 의존성: domain, application, infrastructure, iced 0.13.1
├── Makefile.toml       # 빌드 작업 자동화
└── rustfmt.toml        # Rust 포맷 설정
```

## 주요 의존성

- **iced 0.13.1**: Elm 아키텍처 기반 크로스 플랫폼 GUI 프레임워크
- **rusqlite 0.32**: SQLite 데이터베이스 (bundled 피처)
- **serde 1.0**: 직렬화/역직렬화

## 사전 준비사항

- Rust (최신 stable 버전 권장)
- Cargo (Rust와 함께 설치됨)

## 설치 방법

1. 프로젝트 클론 또는 생성:
```shell
git clone <repository-url>
cd iced-app
```

2. 의존성은 Cargo가 자동으로 처리합니다.

## 실행 방법

```shell
cargo run -p presentation
```

## 문제 해결

### 링커 오류: "invalid linker name in argument '-fuse-ld=mold'"

컴파일 중 이 오류가 발생하면 `mold` 링커가 설정되어 있지만 설치되지 않은 것입니다. 다음과 같이 설치하세요:

**Fedora/RHEL:**
```shell
sudo dnf install mold
```

**Ubuntu/Debian:**
```shell
sudo apt install mold
```

**Arch Linux:**
```shell
sudo pacman -S mold
```

또는 Rust 설정 파일에서 링커 설정을 제거하여 mold를 비활성화할 수 있습니다.

## 참고 자료

- [Iced 예제 코드](https://redandgreen.co.uk/iced-rs-example-snippets/rust-programming/)
- [Iced 공식 문서](https://docs.rs/iced/)
