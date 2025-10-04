# 문제 해결 가이드 (Troubleshooting Guide)

## 일반적인 문제 (Common Issues)

### 1. 컴파일 오류: "DATABASE_URL must be set"

**문제**: 환경 변수가 설정되지 않음

**해결**:
```bash
# .env 파일 생성
cp .env.example .env

# 또는 직접 설정
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
```

### 2. 포트 3000이 이미 사용 중

**문제**: 다른 프로세스가 포트 3000을 사용 중

**해결**:
```bash
# 포트를 사용 중인 프로세스 찾기
lsof -i :3000

# 프로세스 종료
kill -9 <PID>

# 또는 main.rs에서 다른 포트로 변경
# let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
```

### 3. Docker 빌드 실패

**문제**: 의존성 다운로드 또는 빌드 오류

**해결**:
```bash
# Docker 캐시 정리
docker-compose -f docker/docker-compose.yaml down -v
docker system prune -a

# 다시 빌드
docker-compose -f docker/docker-compose.yaml up --build
```

### 4. 데이터베이스 연결 실패

**문제**: PostgreSQL이 실행되지 않거나 연결 정보가 잘못됨

**해결**:
```bash
# PostgreSQL 상태 확인
docker ps | grep postgres

# PostgreSQL 로그 확인
docker logs postgresql

# 연결 테스트
psql postgres://postgres:postgres@localhost:5432/postgres
```

### 5. 마이그레이션 오류

**문제**: 데이터베이스 스키마가 코드와 맞지 않음

**해결**:
```bash
# Diesel CLI 설치
cargo install diesel_cli --no-default-features --features postgres

# 마이그레이션 재실행
diesel migration redo --config-file infra/diesel.toml

# 또는 데이터베이스 초기화
docker-compose -f docker/docker-compose.yaml down -v
docker-compose -f docker/docker-compose.yaml up
```

### 6. Static 파일이 로드되지 않음

**문제**: 웹 UI가 표시되지 않거나 스타일이 적용되지 않음

**해결**:
```bash
# static 디렉토리 확인
ls -la main/static/

# 파일이 있는지 확인
# - index.html
# - style.css
# - app.js

# Docker의 경우 이미지 재빌드
docker-compose -f docker/docker-compose.yaml up --build
```

### 7. CORS 오류 (브라우저 콘솔)

**문제**: 다른 도메인에서 API 호출 시 CORS 오류

**해결**: `main/src/web/mod.rs`에 CORS 미들웨어 추가
```rust
use tower_http::cors::CorsLayer;

let app = Router::new()
    .nest("/api", routes::api_routes())
    .layer(CorsLayer::permissive())  // 개발용
    .with_state(db_conn);
```

### 8. 메모리 부족 오류

**문제**: 빌드 중 메모리 부족

**해결**:
```bash
# 병렬 빌드 작업 수 제한
cargo build --release -j 2

# 또는 Docker에 더 많은 메모리 할당
# Docker Desktop > Settings > Resources > Memory
```

## 디버깅 팁 (Debugging Tips)

### 로그 레벨 증가
```bash
# 더 자세한 로그 출력
export RUST_LOG=debug
cargo run -p main

# 또는 Docker에서
docker-compose -f docker/docker-compose.yaml up
# docker-compose.yaml의 RUST_LOG를 debug로 변경
```

### API 테스트
```bash
# curl로 직접 테스트
curl -v http://localhost:3000/api/todos

# JSON 응답 포맷팅
curl http://localhost:3000/api/todos | jq '.'

# 테스트 스크립트 실행
./test_api.sh
```

### 데이터베이스 직접 확인
```bash
# PostgreSQL 컨테이너 접속
docker exec -it postgresql psql -U postgres

# SQL 실행
\dt                    # 테이블 목록
SELECT * FROM todo;    # 데이터 조회
\q                     # 종료
```

### 코드 포맷 확인
```bash
# 코드 포맷팅
cargo fmt --all

# 린트 검사
cargo clippy --all-targets --all-features
```

## 성능 문제 (Performance Issues)

### 느린 응답 시간

1. **데이터베이스 인덱스 확인**
```sql
CREATE INDEX idx_todo_id ON todo(id);
```

2. **연결 풀 사용** (현재는 단일 연결)
   - `infra/src/lib.rs`에서 r2d2 연결 풀 구현 고려

3. **쿼리 최적화**
   - N+1 쿼리 문제 확인
   - EXPLAIN ANALYZE로 쿼리 분석

## 도움 받기 (Getting Help)

문제가 해결되지 않으면:

1. GitHub Issues에 문제 보고
2. 로그 파일 첨부
3. 환경 정보 제공:
   - OS 버전
   - Rust 버전 (`rustc --version`)
   - Docker 버전 (`docker --version`)
   - 오류 메시지 전문

## 유용한 명령어 (Useful Commands)

```bash
# 전체 재빌드
cargo clean && cargo build --release

# 특정 크레이트만 빌드
cargo build -p main

# 테스트 실행
cargo test --all

# 의존성 트리 확인
cargo tree

# 사용하지 않는 의존성 확인
cargo +nightly udeps

# 빌드 시간 분석
cargo build --release --timings
```
