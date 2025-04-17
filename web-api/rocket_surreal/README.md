# Rocket with SurrealDB (RocksDB Storage)

이 프로젝트는 Rocket 웹 프레임워크와 SurrealDB를 사용하여 REST API를 구현한 예제입니다. SurrealDB의 스토리지 엔진으로 RocksDB를 사용하여 데이터를 디스크에 영구적으로 저장합니다.

## 기능

- REST API를 통한 CRUD 작업
- RocksDB를 사용한 영구 데이터 저장
- 사용자 인증 및 토큰 발급

## SurrealDB 설치

```bash
brew install surrealdb
```

## SurrealDB 실행

```bash
surreal start --user root --pass root memory
```

## 실행 방법

```bash
# 프로젝트 빌드
cargo build

# 프로젝트 실행
cargo run
```

## API 엔드포인트

- `GET /`: 정적 파일 제공
- `GET /session`: 세션 정보 조회
- `POST /person/<id>`: 새 사용자 생성
- `GET /person/<id>`: 사용자 정보 조회
- `PUT /person/<id>`: 사용자 정보 업데이트
- `DELETE /person/<id>`: 사용자 삭제
- `GET /people`: 모든 사용자 목록 조회
- `GET /new_user`: 새 사용자 생성 및 토큰 발급
- `GET /new_token`: 새 토큰 발급 방법 안내

## 데이터 저장 위치

RocksDB 데이터는 `rocksdb_data` 디렉토리에 저장됩니다. 이 디렉토리는 애플리케이션을 처음 실행할 때 자동으로 생성됩니다.
