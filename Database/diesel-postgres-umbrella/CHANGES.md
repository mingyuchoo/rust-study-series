# 변경 사항 (Changes)

## 웹 서비스로 확장 (Web Service Extension)

이 프로젝트를 CLI 애플리케이션에서 완전한 웹 서비스로 확장했습니다.

### 추가된 기능 (Added Features)

#### 1. 완전한 CRUD API
- ✅ **Create**: POST `/api/todos` - 새로운 Todo 생성
- ✅ **Read**: GET `/api/todos` - 모든 Todo 조회
- ✅ **Read**: GET `/api/todos/:id` - 특정 Todo 조회
- ✅ **Update**: PUT `/api/todos/:id` - Todo 수정
- ✅ **Delete**: DELETE `/api/todos/:id` - Todo 삭제

#### 2. 웹 UI
- 모던하고 반응형 디자인
- 실시간 Todo 관리 (추가, 수정, 삭제)
- 그라디언트 배경과 애니메이션 효과
- 모달 기반 수정 인터페이스
- 한글 UI 지원

#### 3. 아키텍처 개선
- Axum 웹 프레임워크 통합
- Tokio 비동기 런타임
- Thread-safe 데이터베이스 연결 (Arc<Mutex>)
- RESTful API 설계

### 수정된 파일 (Modified Files)

#### Domain Layer
- `domain/Cargo.toml` - serde feature 추가
- `domain/src/entities.rs` - Serialize/Deserialize 지원
- `domain/src/repositories.rs` - get, update, delete 메서드 추가

#### Application Layer
- `application/src/lib.rs` - 새로운 use case 모듈 추가
- `application/src/get_todo.rs` - 새 파일: Todo 조회 use case
- `application/src/update_todo.rs` - 새 파일: Todo 수정 use case
- `application/src/delete_todo.rs` - 새 파일: Todo 삭제 use case

#### Infrastructure Layer
- `infra/src/lib.rs` - get_todo, update_todo, delete_todo 함수 추가

#### Adapters Layer
- `adapters/src/persistence_repo.rs` - 새로운 repository 메서드 구현

#### Main Application
- `main/Cargo.toml` - 웹 프레임워크 의존성 추가
- `main/src/main.rs` - 웹 서버로 전환
- `main/src/web/mod.rs` - 새 파일: 웹 모듈
- `main/src/web/routes.rs` - 새 파일: API 라우팅
- `main/src/web/handlers.rs` - 새 파일: HTTP 핸들러

#### Static Files (Web UI)
- `main/static/index.html` - 웹 UI HTML
- `main/static/style.css` - 스타일시트
- `main/static/app.js` - 프론트엔드 JavaScript

#### Docker
- `docker/Dockerfile` - 포트 노출 및 static 파일 복사
- `docker/docker-compose.yaml` - 포트 매핑 추가

#### Documentation
- `README.md` - 웹 서비스 문서 업데이트
- `CHANGES.md` - 이 파일
- `test_api.sh` - API 테스트 스크립트
- `.env.example` - 환경 변수 예제

### 의존성 추가 (New Dependencies)

```toml
axum = "0.7"              # 웹 프레임워크
tokio = "1"               # 비동기 런타임
tower = "0.4"             # 미들웨어
tower-http = "0.5"        # HTTP 유틸리티
serde = "1.0"             # 직렬화/역직렬화
serde_json = "1.0"        # JSON 지원
```

### 사용 방법 (Usage)

#### Docker로 실행
```bash
docker-compose -f docker/docker-compose.yaml up --build
```

#### 로컬 실행
```bash
cargo run -p main
```

#### 접속
- 웹 UI: http://localhost:3000
- API: http://localhost:3000/api/todos

#### API 테스트
```bash
./test_api.sh
```

### 아키텍처 다이어그램

```
Web Browser
    ↓
[Static Files] ← → [REST API]
                      ↓
              [Web Handlers]
                      ↓
              [Use Cases]
                      ↓
              [Domain Entities]
                      ↓
              [Repository Trait]
                      ↓
         [Repository Implementation]
                      ↓
              [Database (PostgreSQL)]
```

### 다음 단계 (Next Steps)

향후 추가할 수 있는 기능:
- [ ] Todo 완료 상태 (completed field)
- [ ] Todo 우선순위
- [ ] 사용자 인증
- [ ] Todo 필터링 및 검색
- [ ] 페이지네이션
- [ ] WebSocket을 통한 실시간 업데이트
- [ ] 단위 테스트 및 통합 테스트
- [ ] API 문서 (OpenAPI/Swagger)
