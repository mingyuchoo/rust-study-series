# 요구사항 관리 문서

## 요구사항 목록

| ID | 구분 | 제목 | 상태 | 영향 파일 | 등록일 |
|---|---|---|---|---|---|
| REQ-R001 | R(구축) | Loco Framework 프로젝트 초기 구축 | 완료 | CLAUDE.md, docs/requirements.md, docs/test-scenarios.md | 2026-02-07 |
| REQ-R002 | R(구축) | Cargo Workspace 모노레포 구조 전환 | 완료 | CLAUDE.md, docs/requirements.md | 2026-02-07 |
| REQ-F001 | F(기능) | 음악 트랙 등록 및 관리 | 완료 | crates/migration, crates/app/src/models, controllers, services, views, tests | 2026-02-07 |
| REQ-F002 | F(기능) | 음악 투표(upvote/downvote) 시스템 | 완료 | crates/migration, crates/app/src/models, controllers, services, tests | 2026-02-07 |
| REQ-F003 | F(기능) | 음악 댓글 시스템 | 완료 | crates/migration, crates/app/src/models, controllers, services, views, tests | 2026-02-07 |
| REQ-F004 | F(기능) | 웹 프론트엔드 (Tera SSR + HTMX + 오디오 플레이어) | 완료 | crates/app/assets/views, static, controllers | 2026-02-07 |
| REQ-N001 | N(비기능) | README.md 프로젝트 문서 작성 | 완료 | README.md | 2026-02-07 |
| REQ-N002 | N(비기능) | 샘플 시드 데이터 생성 (100 회원, 100 트랙, 100 댓글) | 완료 | crates/app/src/tasks/seed_data.rs, crates/app/src/tasks/mod.rs, crates/app/src/app.rs, crates/app/tests/tasks/seed_data.rs | 2026-02-07 |
| REQ-N003 | N(비기능) | 브라우저 E2E 테스트 러너 (17개 시나리오) | 완료 | crates/app/assets/static/e2e-test-runner.html | 2026-02-07 |
| REQ-N004 | N(비기능) | 음악 재생 플레이어 E2E 테스트 시나리오 | 완료 | docs/test-scenarios.md, crates/app/assets/static/e2e-test-runner.html | 2026-02-07 |
| REQ-N005 | N(비기능) | 시드 데이터 음악 링크를 실제 유튜브 링크로 변경 | 완료 | crates/app/src/tasks/seed_data.rs | 2026-02-07 |
| REQ-N006 | N(비기능) | 시드 데이터 제목/아티스트를 실제 YouTube 영상 정보로 교체 | 완료 | crates/app/src/tasks/seed_data.rs | 2026-02-07 |
| REQ-N007 | N(비기능) | 테스트 시나리오를 브라우저 E2E 방식으로 전환 | 완료 | docs/test-scenarios.md | 2026-02-07 |

---

## REQ-R001: Loco Framework 프로젝트 초기 구축

- **구분**: R (구축/Setup)
- **상태**: 진행중
- **설명**: Rust + Loco Framework + Tera + SQLite 기반의 웹 애플리케이션 프로젝트를 초기 구축한다. 프로젝트 관리 문서(CLAUDE.md, requirements.md, test-scenarios.md)를 정비하고, Loco 프로젝트 스캐폴딩을 수행한다.
- **세부 요구사항**:
  1. 프로젝트 관리 문서 정비 (CLAUDE.md, requirements.md, test-scenarios.md)
  2. Loco Framework SaaS App 스캐폴딩 (`loco new`)
  3. Cargo.toml 의존성 설정 (sea-orm, tera, validator, insta, serial_test 등)
  4. 개발/테스트/운영 환경 설정 파일 구성 (config/*.yaml)
  5. SQLite 데이터베이스 연결 설정
  6. Tera 템플릿 엔진 초기화 설정
  7. 기본 디렉토리 구조 생성 확인
  8. `cargo build` 및 `cargo test` 정상 동작 확인
- **영향 파일**:
  - `CLAUDE.md` — 프로젝트 규칙 및 컨벤션 정비
  - `docs/requirements.md` — 요구사항 등록
  - `docs/test-scenarios.md` — 테스트 시나리오 프레임워크 구성
  - `Cargo.toml` — 프로젝트 의존성 (스캐폴딩 후 생성)
  - `config/development.yaml` — 개발 환경 설정
  - `config/test.yaml` — 테스트 환경 설정
  - `src/app.rs` — 앱 등록 및 Hooks (스캐폴딩 후 생성)
  - `src/lib.rs` — 라이브러리 엔트리포인트 (스캐폴딩 후 생성)

---

## REQ-R002: Cargo Workspace 모노레포 구조 전환

- **구분**: R (구축/Setup)
- **상태**: 완료
- **설명**: 기존 단일 크레이트 프로젝트 구조를 Cargo Workspace 기반 모노레포 구조로 전환한다. CLAUDE.md의 프로젝트 구조, 아키텍처 패턴, CLI 참조, 파일 경로 참조를 모노레포 체계에 맞게 갱신한다.
- **세부 요구사항**:
  1. CLAUDE.md 프로젝트 개요에 모노레포 구조 명시
  2. 아키텍처 패턴에 워크스페이스 크레이트 간 의존성 규칙 추가
  3. 디렉토리 구조를 Cargo Workspace 기반으로 전환
  4. 테스트 컨벤션 경로를 워크스페이스 구조에 맞게 갱신
  5. CLI 참조에 워크스페이스 명령어 추가
  6. 테스트 시나리오·워크플로우·금지사항의 파일 경로 갱신
- **영향 파일**:
  - `CLAUDE.md` — 프로젝트 구조 및 컨벤션 전면 갱신
  - `docs/requirements.md` — 요구사항 등록

---

## REQ-F001: 음악 트랙 등록 및 관리

- **구분**: F (Feature)
- **상태**: 완료
- **설명**: 회원이 외부 음악 링크(YouTube, SoundCloud 등)를 저장하고 관리할 수 있다. 트랙은 기본 비공개로 등록되며, 소유자가 공개로 전환할 수 있다. 공개된 트랙은 모든 회원이 조회할 수 있다.
- **세부 요구사항**:
  1. tracks 테이블 마이그레이션 생성 (user_id, title, artist, url, description, is_public, vote_score)
  2. Track SeaORM 엔티티 자동 생성
  3. Track 모델 확장 (Validator: title 필수, url 필수/URL 형식)
  4. TrackService 트레이트 + 구현체 (CRUD + 공개/비공개 토글 + 소유자 권한 검증)
  5. Track 뷰 응답 구조체 (TrackResponse, TrackListResponse)
  6. Track REST API 컨트롤러 (목록, 상세, 등록, 수정, 삭제, 공개 전환, 내 트랙)
  7. 컨트롤러 HTTP 테스트 + 모델 테스트
- **영향 파일**:
  - `crates/migration/src/m20260207_000002_create_tracks.rs`
  - `crates/app/src/models/_entities/tracks.rs` (자동 생성)
  - `crates/app/src/models/tracks.rs`
  - `crates/app/src/services/track_service.rs`
  - `crates/app/src/views/track.rs`
  - `crates/app/src/controllers/track.rs`
  - `crates/app/tests/requests/track.rs`

---

## REQ-F002: 음악 투표(upvote/downvote) 시스템

- **구분**: F (Feature)
- **상태**: 완료
- **설명**: 회원이 공개된 음악 트랙에 대해 upvote 또는 downvote를 할 수 있다. 사용자당 트랙 1개에 1표만 가능하며, 기존 투표를 변경하거나 취소할 수 있다. 투표 점수는 track.vote_score에 동기화된다.
- **세부 요구사항**:
  1. votes 테이블 마이그레이션 생성 (track_id, user_id, vote_type + UNIQUE 인덱스)
  2. Vote 엔티티/모델 생성
  3. VoteService (투표 생성/변경/취소 + vote_score 트랜잭션 갱신)
  4. Vote REST API 컨트롤러 (투표, 투표 취소)
  5. 투표 HTTP 테스트 (신규, 변경, 취소, 중복 방지)
- **영향 파일**:
  - `crates/migration/src/m20260207_000003_create_votes.rs`
  - `crates/app/src/models/_entities/votes.rs` (자동 생성)
  - `crates/app/src/models/votes.rs`
  - `crates/app/src/services/vote_service.rs`
  - `crates/app/src/controllers/vote.rs`
  - `crates/app/tests/requests/vote.rs`

---

## REQ-F003: 음악 댓글 시스템

- **구분**: F (Feature)
- **상태**: 완료
- **설명**: 회원이 공개된 음악 트랙에 댓글을 작성하여 의견을 공유할 수 있다. 댓글은 작성자만 삭제할 수 있다.
- **세부 요구사항**:
  1. comments 테이블 마이그레이션 생성 (track_id, user_id, content)
  2. Comment 엔티티/모델 생성 (Validator: content 필수, 최소 1자)
  3. CommentService (목록 조회, 생성, 삭제 + 작성자 권한 검증)
  4. Comment REST API 컨트롤러 + 뷰 응답 구조체
  5. 댓글 HTTP 테스트
- **영향 파일**:
  - `crates/migration/src/m20260207_000004_create_comments.rs`
  - `crates/app/src/models/_entities/comments.rs` (자동 생성)
  - `crates/app/src/models/comments.rs`
  - `crates/app/src/services/comment_service.rs`
  - `crates/app/src/controllers/comment.rs`
  - `crates/app/src/views/comment.rs`
  - `crates/app/tests/requests/comment.rs`

---

## REQ-F004: 웹 프론트엔드 (Tera SSR + HTMX + 오디오 플레이어)

- **구분**: F (Feature)
- **상태**: 완료
- **설명**: Tera 서버사이드 렌더링 + HTMX를 사용하여 웹 프론트엔드를 구현한다. 공개 트랙 목록, 트랙 상세(투표/댓글), 내 트랙 관리, 인증 페이지를 제공하며, 외부 링크 기반 오디오 플레이어(무한 루프)를 구현한다.
- **세부 요구사항**:
  1. 기본 레이아웃 템플릿 (base.html + nav + HTMX CDN)
  2. 인증 페이지 (로그인, 회원가입)
  3. 홈 페이지 (공개 트랙 목록 + 오디오 플레이어)
  4. 트랙 상세 페이지 (투표 버튼, 댓글 목록/작성, embed 플레이어)
  5. 내 트랙 관리 페이지 (등록 폼, 수정 폼, 공개/비공개 토글)
  6. 오디오 플레이어 JavaScript (iframe embed + 무한 루프)
  7. CSS 스타일링
- **영향 파일**:
  - `crates/app/assets/views/layouts/base.html`
  - `crates/app/assets/views/auth/login.html`, `register.html`
  - `crates/app/assets/views/home/index.html`
  - `crates/app/assets/views/tracks/show.html`, `my.html`, `new.html`, `edit.html`
  - `crates/app/assets/static/css/style.css`
  - `crates/app/assets/static/js/player.js`
  - `crates/app/src/controllers/web_auth.rs`, `web_home.rs`

---

## REQ-N002: 샘플 시드 데이터 생성

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: 앱 실행 시 샘플로 사용할 수 있는 시드 데이터를 Loco CLI Task로 생성한다. 회원 100명, 음악 트랙 100개(공개 상태), 공개 트랙에 달린 댓글 100개를 프로그래밍 방식으로 생성한다.
- **세부 요구사항**:
  1. Loco CLI Task `seed_data` 구현 (`cargo loco task seed_data`로 실행)
  2. 회원 100명 생성 (이메일: user{N}@example.com, 비밀번호: password123)
  3. 음악 트랙 100개 생성 (다양한 제목/아티스트, YouTube URL, 공개 상태)
  4. 공개 트랙에 댓글 100개 생성 (다양한 사용자가 작성)
  5. 비밀번호 해시를 1회만 계산하여 재사용 (Argon2id 성능 최적화)
  6. 태스크 테스트 작성
- **영향 파일**:
  - `crates/app/src/tasks/seed_data.rs` — 시드 데이터 태스크 구현
  - `crates/app/src/tasks/mod.rs` — 태스크 모듈 등록
  - `crates/app/src/app.rs` — 태스크 등록
  - `crates/app/tests/tasks/seed_data.rs` — 태스크 테스트

---

## REQ-N003: 브라우저 E2E 테스트 러너

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: `docs/test-scenarios.md`에 정의된 17개 테스트 시나리오를 웹 브라우저에서 실행할 수 있는 HTML 기반 E2E 테스트 러너를 구현한다. 시드 데이터 사용자로 로그인 후 TC-TRK(7), TC-VOTE(6), TC-CMT(4) 전체 시나리오를 자동 실행하고 결과를 화면에 표시한다.
- **세부 요구사항**:
  1. HTML + JavaScript 기반 자체 포함(self-contained) 테스트 러너 페이지
  2. 시드 데이터 사용자(user1@example.com / password123)로 자동 로그인
  3. TC-TRK-001~007: 트랙 관리 시나리오 7건 자동 실행
  4. TC-VOTE-001~006: 투표 시나리오 6건 자동 실행
  5. TC-CMT-001~004: 댓글 시나리오 4건 자동 실행
  6. 각 시나리오별 통과/실패 결과 및 상세 로그 표시
  7. Playwright MCP를 통한 브라우저 자동 실행 및 결과 확인
- **영향 파일**:
  - `crates/app/assets/static/e2e-test-runner.html` — E2E 테스트 러너 페이지

---

## REQ-N004: 음악 재생 플레이어 E2E 테스트 시나리오

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: 사용자가 로그인하여 등록된 음악의 Play 버튼을 클릭했을 때 iframe 기반 embed 플레이어가 정상 동작하는지 검증하는 E2E 테스트 시나리오를 추가한다. 플레이어 바 표시, iframe embed URL 설정, 트랙 제목/아티스트 표시, 재생 상태 전환을 검증한다.
- **세부 요구사항**:
  1. `docs/test-scenarios.md`에 TC-TRK-008 음악 재생 플레이어 동작 확인 시나리오 추가
  2. `e2e-test-runner.html`에 플레이어 동작 검증 테스트 코드 추가
  3. Playwright 브라우저 E2E 테스트로 실제 동작 검증
- **영향 파일**:
  - `docs/test-scenarios.md` — 시나리오 추가
  - `crates/app/assets/static/e2e-test-runner.html` — E2E 테스트 코드 추가

---

## REQ-N005: 시드 데이터 음악 링크를 실제 유튜브 링크로 변경

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: 시드 데이터의 더미 YouTube URL(`seed_0001`~`seed_0100`)을 실제 동작하는 유튜브 음악 영상 링크로 교체한다. YouTube 최다 조회 음악 영상 100개의 실제 영상 ID를 사용한다.
- **세부 요구사항**:
  1. `YOUTUBE_IDS` 상수 배열에 실제 유튜브 영상 ID 100개 정의
  2. 기존 `seed_{i:04}` 형식의 더미 URL을 실제 영상 ID 기반 URL로 교체
  3. 기존 테스트 통과 확인
- **영향 파일**:
  - `crates/app/src/tasks/seed_data.rs` — 시드 데이터 URL 교체

---

## REQ-N006: 시드 데이터 제목/아티스트를 실제 YouTube 영상 정보로 교체

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: 시드 데이터의 가상 제목(Midnight Dreams 등)과 가짜 아티스트(The Wanderers 등)를 실제 YouTube 영상의 곡명과 아티스트로 교체한다. YouTube ID별 실제 곡 정보(제목, 아티스트, 장르)를 매핑하여 시드 데이터의 정합성을 확보한다.
- **세부 요구사항**:
  1. `TITLE_PREFIXES`, `TITLE_SUFFIXES`, `ARTISTS`, `GENRES`, `YOUTUBE_IDS` 별도 배열을 하나의 통합 배열로 교체
  2. 각 YouTube ID에 실제 곡명, 아티스트명, 장르를 매핑
  3. 트랙 생성 시 통합 배열에서 실제 정보 사용
  4. description 필드에 실제 장르 및 아티스트 정보 반영
  5. 기존 테스트 통과 확인
- **영향 파일**:
  - `crates/app/src/tasks/seed_data.rs` — 시드 데이터 트랙 정보 교체

---

## REQ-N007: 테스트 시나리오를 브라우저 E2E 방식으로 전환

- **구분**: N (Non-functional)
- **상태**: 완료
- **설명**: `docs/test-scenarios.md`의 테스트 시나리오를 API 직접 호출 방식에서 실제 웹 브라우저에서 페이지를 방문하고 UI를 조작하는 사용자 관점의 E2E 테스트 방식으로 전환한다. 인증(TC-AUTH) 시나리오를 신규 추가하고, 기존 시나리오의 테스트 단계를 브라우저 동작(페이지 이동, 폼 입력, 버튼 클릭, 화면 확인)으로 변경한다.
- **세부 요구사항**:
  1. TC-AUTH 인증 시나리오 신규 추가 (회원가입, 로그인, 로그아웃)
  2. TC-TRK 트랙 관리 시나리오를 웹 페이지 UI 조작 방식으로 전환
  3. TC-VOTE 투표 시나리오를 웹 페이지 UI 조작 방식으로 전환 (UI 미지원 기능은 API 전용으로 분류)
  4. TC-CMT 댓글 시나리오를 웹 페이지 UI 조작 방식으로 전환
  5. 시나리오 통계 및 코드 소스별 매핑표 갱신
- **영향 파일**:
  - `docs/test-scenarios.md` — 전체 시나리오 브라우저 E2E 방식으로 전환

---

## 요구사항 구분 코드

| 코드 | 의미 | 설명 |
|------|------|------|
| F | Feature | 신규 기능 개발 |
| N | Non-functional | 비기능 요구사항 (성능, 보안 등) |
| B | Bugfix | 버그 수정 |
| R | Refactoring/Setup | 리팩토링 또는 프로젝트 구축 |

---
