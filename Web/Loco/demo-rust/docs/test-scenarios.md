# 사용자 테스트 시나리오 문서

## 1. 개요

### 목적

코드베이스를 분석하여 **사용자 관점의 브라우저 E2E 테스트 시나리오**를 체계적으로 도출한 문서입니다.
모든 시나리오는 **실제 웹 브라우저에서 페이지를 방문하고 UI를 직접 조작**하여 검증합니다.
컨트롤러, 모델 검증 규칙(Validator), Service 비즈니스 로직, 인증/보안 설정을 분석하여 완전한 시나리오 세트를 도출합니다.

### 테스트 방식

| 구분 | 설명 | 도구 |
|------|------|------|
| **브라우저 UI 테스트** | 실제 웹 페이지에서 폼 입력, 버튼 클릭, 화면 확인 | 웹 브라우저 (Chrome 등) |
| **API 전용 테스트** | UI에서 직접 수행 불가능한 시나리오 (엣지 케이스) | E2E 테스트 러너 (`/static/e2e-test-runner.html`) |

### 추출 방법론

| 소스 | 추출 대상 | 분석 파일 패턴 |
|------|----------|---------------|
| 컨트롤러 (`src/controllers/`) | 사용자 동작(Action) 흐름 | `routes()` 함수, `get`/`post`/`delete`/`patch` 핸들러 |
| 웹 컨트롤러 (`src/controllers/web.rs`) | 브라우저 페이지 흐름 | 웹 페이지 라우트, Tera 템플릿 렌더링 |
| 웹 템플릿 (`assets/views/`) | UI 요소 및 사용자 인터랙션 | HTML 폼, 버튼, JavaScript 이벤트 |
| 모델 검증 규칙 (`src/models/`) | 입력 유효성 시나리오 | `#[derive(Validate)]`, `#[validate(...)]` 어트리뷰트 |
| Service 비즈니스 로직 (`src/services/`) | 예외/에러 시나리오 | `Err(Error::...)` 반환 지점 |
| 인증/보안 설정 (`config/`, `src/app.rs`) | 보안/인증 시나리오 | JWT/auth 설정, 라우트 인증 미들웨어 |

### 시나리오 분류 체계

- **TC-AUTH**: 인증 (회원가입, 로그인, 로그아웃)
- **TC-TRK**: 트랙 관리 (CRUD, 공개/비공개, 재생)
- **TC-VOTE**: 투표 관리 (upvote/downvote)
- **TC-CMT**: 댓글 관리 (생성, 삭제)
- **TC-SEC**: 권한/보안
- **TC-VAL**: 입력 유효성 검증

### 웹 페이지 구조

| 페이지 | 경로 | 인증 필요 | 주요 기능 |
|--------|------|----------|----------|
| 홈 | `/` | - | 공개 트랙 목록, 투표, 재생 |
| 로그인 | `/auth/login` | - | 이메일/비밀번호 로그인 |
| 회원가입 | `/auth/register` | - | 이름/이메일/비밀번호 등록 |
| 트랙 상세 | `/tracks/{id}` | - | 트랙 정보, 투표, 댓글 |
| 내 트랙 관리 | `/my/tracks` | O | 본인 트랙 목록, 공개/비공개, 수정, 삭제 |
| 트랙 추가 | `/tracks/new` | O | 새 트랙 등록 폼 |
| 트랙 수정 | `/tracks/{id}/edit` | O | 트랙 정보 수정 폼 |

---

## 2. 사전 조건

### 테스트 환경

- **Base URL**: `http://localhost:5150` (Loco 기본 포트)
- **DB**: SQLite (테스트용 인메모리 또는 파일 DB)
- **설정 파일**: `config/development.yaml`
- **인증 방식**: JWT Bearer Token (localStorage에 저장)
- **브라우저**: Chrome, Firefox 등 최신 브라우저

### 테스트 환경 준비

```bash
# 1. DB 마이그레이션 실행
cargo loco db migrate

# 2. 시드 데이터 로드
cargo loco task seed_data

# 3. 개발 서버 실행
cargo loco start --environment development

# 4. 브라우저에서 접속
# http://localhost:5150/
```

### 샘플 계정 (시드 데이터)

| 역할 | 이메일 | 비밀번호 | 비고 |
|------|--------|----------|------|
| 일반 사용자 | user1@example.com | password123 | 기본 테스트 계정 |
| 일반 사용자 | user2@example.com | password123 | 다중 사용자 테스트용 |

### 샘플 데이터

- 시드 데이터로 회원 100명, 공개 트랙 100개, 댓글 100개가 생성됩니다.
- 각 테스트 시나리오에서 필요한 사전 데이터는 "사전 조건" 항목에 명시합니다.

---

## 3. 테스트 시나리오

### 시나리오 통계

| 카테고리 | 브라우저 UI | API 전용 | 합계 |
|---------|-----------|---------|------|
| TC-AUTH | 3 | - | 3 |
| TC-TRK  | 8 | - | 8 |
| TC-VOTE | 3 | 3 | 6 |
| TC-CMT  | 4 | - | 4 |
| TC-SEC  | - | - | 0 |
| TC-VAL  | - | - | 0 |
| **합계** | **18** | **3** | **21** |

---

### 3.1 TC-AUTH: 인증

#### TC-AUTH-001: 회원가입

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그아웃 상태 (localStorage에 token 없음)
- **테스트 단계**:
  1. 브라우저에서 `http://localhost:5150/auth/register` 페이지 접속
  2. "Register" 제목이 표시되는지 확인
  3. Name 입력란에 `Test User` 입력
  4. Email 입력란에 `testuser@example.com` 입력
  5. Password 입력란에 `password123` 입력
  6. "Register" 버튼 클릭
  7. `/auth/login` 페이지로 리다이렉트되는지 확인
- **기대 결과**:
  - 회원가입 성공 후 로그인 페이지(`/auth/login`)로 자동 이동
  - 에러 메시지가 표시되지 않음
- **코드 근거**: `crates/app/assets/views/auth/register.html:34-56` (폼 제출 JS), `crates/app/src/controllers/auth.rs` (API 처리)

#### TC-AUTH-002: 로그인

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 시드 데이터의 사용자 계정 존재
- **테스트 단계**:
  1. 브라우저에서 `http://localhost:5150/auth/login` 페이지 접속
  2. "Login" 제목이 표시되는지 확인
  3. Email 입력란에 `user1@example.com` 입력
  4. Password 입력란에 `password123` 입력
  5. "Login" 버튼 클릭
  6. 홈 페이지(`/`)로 리다이렉트되는지 확인
  7. 네비게이션 바에 사용자 이름이 표시되는지 확인
  8. 네비게이션 바에 "My Tracks", "Add Track" 링크가 표시되는지 확인
- **기대 결과**:
  - 로그인 성공 후 홈 페이지(`/`)로 자동 이동
  - 네비게이션 바가 로그인 상태로 변경 (사용자 이름, My Tracks, Add Track, Logout 표시)
  - localStorage에 `token`과 `user` 정보 저장
- **코드 근거**: `crates/app/assets/views/auth/login.html:30-52` (폼 제출 JS), `crates/app/assets/static/js/player.js:76-95` (네비 업데이트)

#### TC-AUTH-003: 로그아웃

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 상태 (TC-AUTH-002 완료)
- **테스트 단계**:
  1. 네비게이션 바에서 "Logout" 링크 클릭
  2. 네비게이션 바가 로그아웃 상태로 변경되는지 확인
  3. "Login", "Register" 링크가 다시 표시되는지 확인
- **기대 결과**:
  - 네비게이션 바가 비로그인 상태로 복원 (Login, Register 표시)
  - localStorage에서 `token`과 `user` 제거
- **코드 근거**: `crates/app/assets/static/js/player.js:76-95` (네비 업데이트), `crates/app/assets/views/layouts/base.html:20` (Logout 버튼)

---

### 3.2 TC-TRK: 트랙 관리

#### TC-TRK-001: 트랙 등록

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자 (TC-AUTH-002 완료)
- **테스트 단계**:
  1. 네비게이션 바에서 "Add Track" 링크 클릭
  2. `/tracks/new` 페이지로 이동 확인
  3. "Add New Track" 제목이 표시되는지 확인
  4. Title 입력란에 `My Test Song` 입력
  5. Artist 입력란에 `Test Artist` 입력
  6. URL 입력란에 `https://www.youtube.com/watch?v=dQw4w9WgXcQ` 입력
  7. Description 입력란에 `A great test track` 입력
  8. "Add Track" 버튼 클릭
  9. `/my/tracks` 페이지로 리다이렉트되는지 확인
  10. 목록에 `My Test Song` 트랙이 표시되는지 확인
  11. 상태가 "Private"로 표시되는지 확인
- **기대 결과**:
  - 트랙 등록 성공 후 내 트랙 관리 페이지(`/my/tracks`)로 자동 이동
  - 새 트랙이 목록에 표시되며 기본 상태는 "Private"
- **코드 근거**: `crates/app/assets/views/tracks/new.html:32-58` (폼 제출), `crates/app/src/models/tracks.rs:91-107` (모델 생성)

#### TC-TRK-002: 공개 트랙 목록 조회

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 공개 전환된 트랙이 1개 이상 존재 (시드 데이터)
- **테스트 단계**:
  1. 브라우저에서 `http://localhost:5150/` 홈 페이지 접속 (인증 불필요)
  2. "Public Tracks" 제목이 표시되는지 확인
  3. 트랙 카드 목록이 로딩되는지 확인
  4. 각 트랙 카드에 제목, 아티스트, 날짜, 투표 점수가 표시되는지 확인
  5. ▶ (Play) 버튼, ▲ (Upvote) 버튼, ▼ (Downvote) 버튼이 표시되는지 확인
  6. 트랙 목록이 vote_score 내림차순으로 정렬되어 있는지 확인
- **기대 결과**:
  - 공개 트랙만 표시됨 (비공개 트랙은 미표시)
  - vote_score 내림차순 정렬
  - 각 트랙 카드에 제목 링크, 아티스트, 날짜, 투표 UI 표시
- **코드 근거**: `crates/app/assets/views/home/index.html:53-68` (loadTracks), `crates/app/src/models/tracks.rs:65-72` (find_public 쿼리)

#### TC-TRK-003: 공개/비공개 전환

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 본인이 등록한 트랙 (TC-TRK-001 완료)
- **테스트 단계**:
  1. 네비게이션 바에서 "My Tracks" 클릭하여 `/my/tracks` 이동
  2. 본인 트랙 카드에서 현재 상태 확인 ("Private" 표시)
  3. "Make Public" 버튼 클릭
  4. 목록이 새로고침되어 상태가 "Public"으로 변경되고 버튼이 "Make Private"로 변경되는지 확인
  5. "Make Private" 버튼을 다시 클릭
  6. 상태가 "Private"로 복원되고 버튼이 "Make Public"으로 돌아오는지 확인
- **기대 결과**:
  - 첫 클릭: "Private" → "Public", 버튼 텍스트 "Make Private"로 변경
  - 재클릭: "Public" → "Private", 버튼 텍스트 "Make Public"으로 변경
- **코드 근거**: `crates/app/assets/views/tracks/my.html:47-48` (toggleBtn), `crates/app/src/models/tracks.rs:133-137` (토글 모델)

#### TC-TRK-004: 트랙 수정

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 본인이 등록한 트랙
- **테스트 단계**:
  1. `/my/tracks` 페이지에서 수정할 트랙의 "Edit" 버튼 클릭
  2. `/tracks/{id}/edit` 페이지로 이동 확인
  3. "Edit Track" 제목이 표시되는지 확인
  4. 기존 트랙 정보(Title, Artist, URL, Description)가 폼에 자동 채워져 있는지 확인
  5. Title 입력란의 값을 `Updated Song Title`로 변경
  6. "Save Changes" 버튼 클릭
  7. `/my/tracks` 페이지로 리다이렉트되는지 확인
  8. 목록에서 변경된 제목 `Updated Song Title`이 표시되는지 확인
- **기대 결과**:
  - 수정 페이지에 기존 정보가 미리 채워져 표시
  - 저장 후 내 트랙 목록으로 이동하며, 변경사항이 반영됨
- **코드 근거**: `crates/app/assets/views/tracks/edit.html:34-76` (로드 + 수정), `crates/app/src/models/tracks.rs:112-130` (수정 모델)

#### TC-TRK-005: 트랙 삭제

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 본인이 등록한 트랙
- **테스트 단계**:
  1. `/my/tracks` 페이지에서 삭제할 트랙의 제목을 기억
  2. 해당 트랙의 "Delete" 버튼 클릭
  3. 브라우저 확인 대화 상자(confirm)가 "Are you sure?" 메시지로 표시되는지 확인
  4. "확인(OK)" 클릭
  5. 목록이 새로고침되어 해당 트랙이 사라졌는지 확인
- **기대 결과**:
  - 확인 대화 상자 표시 후 "확인" 시 트랙 삭제
  - 목록에서 해당 트랙 제거됨
- **코드 근거**: `crates/app/assets/views/tracks/my.html:52-53` (delBtn + confirm), `crates/app/src/controllers/track.rs:80-94` (삭제 핸들러)

#### TC-TRK-006: 내 트랙 목록 조회

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 본인이 등록한 트랙 1개 이상
- **테스트 단계**:
  1. 네비게이션 바에서 "My Tracks" 링크 클릭
  2. `/my/tracks` 페이지로 이동 확인
  3. "My Tracks" 제목이 표시되는지 확인
  4. "+ Add Track" 버튼이 우측 상단에 표시되는지 확인
  5. 본인이 등록한 트랙 카드가 목록에 표시되는지 확인
  6. 각 카드에 제목, 아티스트, 공개/비공개 상태, 투표 점수가 표시되는지 확인
  7. 각 카드에 "Make Public/Private", "Edit", "Delete" 버튼이 표시되는지 확인
- **기대 결과**:
  - 본인이 등록한 트랙만 표시 (다른 사용자의 트랙 미표시)
  - 각 트랙에 관리 버튼(공개 전환, 수정, 삭제) 표시
- **코드 근거**: `crates/app/assets/views/tracks/my.html:27-59` (loadMyTracks), `crates/app/src/controllers/track.rs:116-120` (my 핸들러)

#### TC-TRK-007: 미인증 상태 트랙 등록 페이지 접근 거부

- **카테고리**: 브라우저 UI — 보안
- **사전 조건**: 로그아웃 상태 (localStorage에 token 없음)
- **테스트 단계**:
  1. 브라우저 주소창에 `http://localhost:5150/tracks/new` 직접 입력하여 접속
  2. 자동으로 `/auth/login` 페이지로 리다이렉트되는지 확인
- **기대 결과**:
  - 트랙 등록 폼이 표시되지 않고, 로그인 페이지로 자동 이동
- **코드 근거**: `crates/app/assets/views/tracks/new.html:35` (`if (!token) redirect`), `crates/app/src/controllers/track.rs:49` (auth::JWT 추출기)

#### TC-TRK-008: 음악 재생 플레이어 동작 확인

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 공개된 트랙이 1개 이상 존재 (시드 데이터)
- **테스트 단계**:
  1. 브라우저에서 `http://localhost:5150/` 홈 페이지 접속
  2. 공개 트랙 목록에서 첫 번째 트랙의 ▶ (Play) 버튼 클릭
  3. 하단 플레이어 바(`#player-bar`)가 표시되는지 확인
  4. 플레이어 제목(`#player-title`)에 트랙 제목이 표시되는지 확인
  5. 플레이어 아티스트(`#player-artist`)에 아티스트명이 표시되는지 확인
  6. `#player-iframe`의 `src`에 YouTube embed URL (`youtube.com/embed/`)이 설정되었는지 확인
  7. `#player-toggle` 버튼이 ⏸ (일시정지) 아이콘으로 변경되는지 확인
- **기대 결과**:
  - 플레이어 바가 화면 하단에 표시됨 (`display: block`)
  - 트랙 제목과 아티스트가 플레이어에 표시됨
  - iframe에 YouTube embed URL이 설정됨 (`youtube.com/embed/{videoId}`)
  - 재생 상태로 전환 (일시정지 버튼 표시)
- **코드 근거**: `crates/app/assets/static/js/player.js:29-56` (play, loadCurrent), `crates/app/assets/views/home/index.html:26-27` (playTrack 호출)

---

### 3.3 TC-VOTE: 투표

#### TC-VOTE-001: 홈 페이지에서 Upvote 투표

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 공개된 트랙이 존재 (시드 데이터)
- **테스트 단계**:
  1. 홈 페이지(`/`)에서 공개 트랙의 현재 투표 점수를 확인 (예: `5`)
  2. 해당 트랙의 ▲ (Upvote) 버튼 클릭
  3. 투표 점수가 `6`으로 증가하는지 확인
- **기대 결과**:
  - 투표 점수가 +1 증가하여 화면에 즉시 반영
- **코드 근거**: `crates/app/assets/views/home/index.html:42-43` (upBtn), `crates/app/src/models/votes.rs:39-82` (투표 트랜잭션)

#### TC-VOTE-002: 트랙 상세 페이지에서 Downvote 투표

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 공개된 트랙
- **테스트 단계**:
  1. 홈 페이지(`/`)에서 트랙 제목 링크를 클릭하여 `/tracks/{id}` 상세 페이지로 이동
  2. 현재 투표 점수를 확인
  3. ▼ (Downvote) 버튼 클릭
  4. 투표 점수가 -1 감소하는지 확인
- **기대 결과**:
  - 투표 점수가 -1 감소하여 화면에 즉시 반영
- **코드 근거**: `crates/app/assets/views/tracks/show.html:53` (downBtn), `crates/app/src/models/votes.rs:39-82` (투표 트랜잭션)

#### TC-VOTE-003: 미인증 상태 투표 시 로그인 페이지 이동

- **카테고리**: 브라우저 UI — 보안
- **사전 조건**: 로그아웃 상태
- **테스트 단계**:
  1. 홈 페이지(`/`)에서 공개 트랙의 ▲ (Upvote) 버튼 클릭
  2. `/auth/login` 페이지로 자동 이동하는지 확인
- **기대 결과**:
  - 투표가 실행되지 않고 로그인 페이지로 리다이렉트
- **코드 근거**: `crates/app/assets/views/home/index.html:73` (`if (!token) redirect`), `crates/app/assets/views/tracks/show.html:71` (동일 로직)

#### TC-VOTE-004: 투표 변경 (upvote → downvote) [API 전용]

- **카테고리**: API 전용 — 정상 경로
- **사전 조건**: 로그인된 사용자, 이미 upvote한 공개 트랙
- **테스트 방법**: E2E 테스트 러너 (`/static/e2e-test-runner.html`) 사용
- **테스트 단계**:
  1. `POST /api/tracks/{id}/vote` 에 `{ "vote_type": -1 }` 전송
  2. 투표가 downvote로 변경되고 vote_score에 diff(-2) 반영 확인
- **기대 결과**:
  - HTTP 200, vote_type=-1 반환, vote_score 차이분 반영
- **코드 근거**: `crates/app/src/models/votes.rs:55-65` (투표 변경 로직)

#### TC-VOTE-005: 유효하지 않은 vote_type 거부 [API 전용]

- **카테고리**: API 전용 — 유효성 검증
- **사전 조건**: 로그인된 사용자, 공개된 트랙
- **테스트 방법**: E2E 테스트 러너 (`/static/e2e-test-runner.html`) 사용
- **테스트 단계**:
  1. `POST /api/tracks/{id}/vote` 에 `{ "vote_type": 2 }` 전송
- **기대 결과**:
  - HTTP 400 Bad Request
- **코드 근거**: `crates/app/src/controllers/vote.rs:22-24` (vote_type 범위 검증)

#### TC-VOTE-006: 투표 취소 [API 전용]

- **카테고리**: API 전용 — 예외 경로
- **사전 조건**: 로그인된 사용자, 이미 투표한 공개 트랙
- **테스트 방법**: E2E 테스트 러너 (`/static/e2e-test-runner.html`) 사용
- **테스트 단계**:
  1. `DELETE /api/tracks/{id}/vote` 호출
  2. vote_score가 원래값으로 복원되었는지 확인
- **기대 결과**:
  - HTTP 200, vote_score 원복
- **코드 근거**: `crates/app/src/controllers/vote.rs:38-48`, `crates/app/src/models/votes.rs:85-101` (투표 취소 트랜잭션)

---

### 3.4 TC-CMT: 댓글 관리

#### TC-CMT-001: 댓글 작성

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 로그인된 사용자, 공개된 트랙
- **테스트 단계**:
  1. 홈 페이지(`/`)에서 트랙 제목 링크를 클릭하여 `/tracks/{id}` 상세 페이지로 이동
  2. "Comments" 섹션이 표시되는지 확인
  3. 댓글 입력 텍스트 영역(`#comment-input`)이 표시되는지 확인 (로그인 상태)
  4. 텍스트 영역에 `Great song! Love it!` 입력
  5. "Post Comment" 버튼 클릭
  6. 텍스트 영역이 비워지는지 확인
  7. 댓글 목록에 방금 작성한 `Great song! Love it!` 댓글이 표시되는지 확인
- **기대 결과**:
  - 댓글 작성 후 텍스트 영역 초기화
  - 댓글 목록에 새 댓글 즉시 표시 (사용자 ID, 날짜 포함)
- **코드 근거**: `crates/app/assets/views/tracks/show.html:117-132` (댓글 제출), `crates/app/src/models/comments.rs:54-65` (댓글 생성 모델)

#### TC-CMT-002: 댓글 목록 조회

- **카테고리**: 브라우저 UI — 정상 경로
- **사전 조건**: 공개된 트랙에 댓글이 존재 (시드 데이터)
- **테스트 단계**:
  1. 브라우저에서 댓글이 존재하는 트랙의 상세 페이지(`/tracks/{id}`) 접속 (인증 불필요)
  2. "Comments" 섹션이 표시되는지 확인
  3. 댓글 목록이 로딩되는지 확인
  4. 각 댓글에 사용자 ID, 날짜, 내용이 표시되는지 확인
- **기대 결과**:
  - 해당 트랙의 댓글이 생성일 내림차순으로 표시
  - 각 댓글에 작성자 정보와 내용이 표시
- **코드 근거**: `crates/app/assets/views/tracks/show.html:87-115` (loadComments), `crates/app/src/models/comments.rs:42-51` (find_by_track 쿼리)

#### TC-CMT-003: 댓글 삭제 (작성자)

- **카테고리**: 브라우저 UI — 예외 경로
- **사전 조건**: 로그인된 사용자, 본인이 작성한 댓글 (TC-CMT-001 완료)
- **테스트 단계**:
  1. 트랙 상세 페이지(`/tracks/{id}`)에서 본인이 작성한 댓글을 찾음
  2. 해당 댓글에 "Delete" 버튼이 표시되는지 확인 (본인 댓글만 삭제 가능)
  3. "Delete" 버튼 클릭
  4. 댓글 목록이 새로고침되어 해당 댓글이 사라졌는지 확인
- **기대 결과**:
  - 본인 댓글에만 "Delete" 버튼 표시
  - 삭제 후 댓글 목록에서 제거됨
- **코드 근거**: `crates/app/assets/views/tracks/show.html:104-109` (Delete 버튼 조건부 표시), `crates/app/src/models/comments.rs:76-81` (댓글 삭제 모델)

#### TC-CMT-004: 미인증 상태 댓글 작성 폼 숨김

- **카테고리**: 브라우저 UI — 보안
- **사전 조건**: 로그아웃 상태
- **테스트 단계**:
  1. 브라우저에서 공개 트랙의 상세 페이지(`/tracks/{id}`) 접속
  2. "Comments" 섹션이 표시되는지 확인
  3. 댓글 입력 텍스트 영역과 "Post Comment" 버튼이 **표시되지 않는지** 확인
  4. 기존 댓글 목록은 정상적으로 표시되는지 확인
- **기대 결과**:
  - 댓글 작성 폼(`#comment-form-box`)이 `display: none` 상태
  - 댓글 읽기(목록 조회)는 비인증 상태에서도 가능
- **코드 근거**: `crates/app/assets/views/tracks/show.html:63-65` (token 체크 후 폼 표시), `crates/app/src/controllers/comment.rs:28` (auth::JWT 추출기)

---

### 3.5 TC-SEC: 권한/보안

> 인증 미들웨어 및 권한 검사 구현 후(REQ-F0XX) 추가됩니다.

---

### 3.6 TC-VAL: 입력 유효성 검증

> Validator 검증 규칙이 모델에 추가된 후(REQ-F0XX) 추가됩니다.

---

## 4. 코드 소스별 시나리오 매핑

| 소스 파일:라인 | 시나리오 ID | 설명 |
|---------------|-----------|------|
| `crates/app/assets/views/auth/register.html:34-56` | TC-AUTH-001 | 회원가입 폼 제출 |
| `crates/app/assets/views/auth/login.html:30-52` | TC-AUTH-002 | 로그인 폼 제출 |
| `crates/app/assets/static/js/player.js:76-95` | TC-AUTH-002, TC-AUTH-003 | 네비 로그인 상태 업데이트 |
| `crates/app/assets/views/layouts/base.html:20` | TC-AUTH-003 | 로그아웃 버튼 |
| `crates/app/assets/views/tracks/new.html:32-58` | TC-TRK-001 | 트랙 등록 폼 |
| `crates/app/assets/views/home/index.html:53-68` | TC-TRK-002 | 홈 페이지 트랙 목록 로드 |
| `crates/app/assets/views/home/index.html:25-51` | TC-TRK-002 | 트랙 카드 빌드 |
| `crates/app/assets/views/tracks/my.html:27-59` | TC-TRK-003, TC-TRK-006 | 내 트랙 목록 및 공개 전환 |
| `crates/app/assets/views/tracks/my.html:47-48` | TC-TRK-003 | 공개/비공개 토글 버튼 |
| `crates/app/assets/views/tracks/edit.html:34-76` | TC-TRK-004 | 트랙 수정 폼 (로드 + 저장) |
| `crates/app/assets/views/tracks/my.html:52-53` | TC-TRK-005 | 트랙 삭제 버튼 + confirm |
| `crates/app/assets/views/tracks/new.html:35` | TC-TRK-007 | 미인증 리다이렉트 |
| `crates/app/assets/static/js/player.js:29-56` | TC-TRK-008 | 음악 재생 플레이어 (play, loadCurrent) |
| `crates/app/assets/views/home/index.html:26-27` | TC-TRK-008 | 홈 페이지 playTrack 호출 |
| `crates/app/assets/views/home/index.html:42-43` | TC-VOTE-001 | Upvote 버튼 (홈) |
| `crates/app/assets/views/home/index.html:71-86` | TC-VOTE-001, TC-VOTE-002 | 투표 fetch + 점수 갱신 (홈) |
| `crates/app/assets/views/tracks/show.html:49-53` | TC-VOTE-002 | Upvote/Downvote 버튼 (상세) |
| `crates/app/assets/views/tracks/show.html:69-84` | TC-VOTE-002 | 투표 fetch + 점수 갱신 (상세) |
| `crates/app/assets/views/home/index.html:73` | TC-VOTE-003 | 미인증 투표 리다이렉트 (홈) |
| `crates/app/assets/views/tracks/show.html:71` | TC-VOTE-003 | 미인증 투표 리다이렉트 (상세) |
| `crates/app/src/models/votes.rs:55-65` | TC-VOTE-004 | 투표 변경 로직 [API 전용] |
| `crates/app/src/controllers/vote.rs:22-24` | TC-VOTE-005 | vote_type 유효성 검증 [API 전용] |
| `crates/app/src/controllers/vote.rs:38-48` | TC-VOTE-006 | 투표 취소 핸들러 [API 전용] |
| `crates/app/src/models/votes.rs:85-101` | TC-VOTE-006 | 투표 취소 트랜잭션 [API 전용] |
| `crates/app/assets/views/tracks/show.html:117-132` | TC-CMT-001 | 댓글 작성 (Post Comment) |
| `crates/app/assets/views/tracks/show.html:87-115` | TC-CMT-002 | 댓글 목록 로드 |
| `crates/app/assets/views/tracks/show.html:104-109` | TC-CMT-003 | 댓글 삭제 버튼 (본인만) |
| `crates/app/assets/views/tracks/show.html:134-143` | TC-CMT-003 | 댓글 삭제 fetch |
| `crates/app/assets/views/tracks/show.html:63-65` | TC-CMT-004 | 미인증 시 댓글 폼 숨김 |
| `crates/app/src/models/tracks.rs:65-72` | TC-TRK-002 | find_public 쿼리 |
| `crates/app/src/models/tracks.rs:91-107` | TC-TRK-001 | 트랙 생성 모델 |
| `crates/app/src/models/tracks.rs:112-130` | TC-TRK-004 | 트랙 수정 모델 |
| `crates/app/src/models/tracks.rs:133-137` | TC-TRK-003 | 공개/비공개 토글 모델 |
| `crates/app/src/models/comments.rs:42-51` | TC-CMT-002 | find_by_track 쿼리 |
| `crates/app/src/models/comments.rs:54-65` | TC-CMT-001 | 댓글 생성 모델 |
| `crates/app/src/models/comments.rs:76-81` | TC-CMT-003 | 댓글 삭제 모델 |
| `crates/app/src/models/votes.rs:39-82` | TC-VOTE-001, TC-VOTE-002 | 투표 생성/변경 트랜잭션 |

---

## 5. 변경 이력

| 날짜 | 변경 내용 | 관련 REQ |
|------|----------|---------|
| 2026-02-07 | 테스트 시나리오 문서 초기 프레임워크 구성 | REQ-R001 |
| 2026-02-07 | TC-TRK 트랙 관리 시나리오 7건 추가 | REQ-F001 |
| 2026-02-07 | TC-VOTE 투표 시나리오 6건 추가 | REQ-F002 |
| 2026-02-07 | TC-CMT 댓글 시나리오 4건 추가 | REQ-F003 |
| 2026-02-07 | TC-TRK-008 음악 재생 플레이어 동작 확인 시나리오 추가 | REQ-N004 |
| 2026-02-07 | 전체 시나리오를 브라우저 E2E 방식으로 전환, TC-AUTH 3건 추가 | REQ-N007 |
