# 테스트 커버리지 100% 달성 설계안

**작성일:** 2026-02-15
**목표:** ratatui-diary 프로젝트의 모든 코드(main.rs 포함)에 대한 100% 테스트 커버리지 달성
**접근 방식:** 반복적 커버리지 개선 (데이터 기반)

---

## 1. 아키텍처 개요

### 전체 구조

```
┌─────────────────────────────────────────┐
│  cargo-llvm-cov (커버리지 측정 도구)     │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│  현재 커버리지 리포트 (HTML)             │
│  - 누락된 라인 식별                      │
│  - 모듈별 커버리지 백분율                │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│  테스트 추가 우선순위                    │
│  1. storage.rs (간단, 기초)              │
│  2. model.rs (핵심 로직)                 │
│  3. update.rs (비즈니스 로직)            │
│  4. view.rs (UI 렌더링)                  │
│  5. markdown.rs (렌더링 유틸)            │
│  6. main.rs (통합 테스트)                │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│  반복 사이클 (모듈당)                    │
│  1. 누락 코드 식별                       │
│  2. BDD 명세 작성                        │
│  3. 테스트 구현 (TDD)                    │
│  4. 커버리지 재측정                      │
│  5. 100% 달성 확인                       │
└─────────────────────────────────────────┘
```

### 핵심 원칙

- **데이터 기반**: 추측이 아닌 측정 데이터로 결정
- **점진적**: 한 번에 하나의 모듈씩 개선
- **검증 가능**: 각 단계마다 커버리지 확인
- **TDD 준수**: CLAUDE.md의 요구사항 준수

---

## 2. 구성 요소

### 2.1 커버리지 측정 인프라

**cargo-llvm-cov 설정:**
- `Cargo.toml`에 추가 설정 불필요 (dev-dependency로만 사용)
- 커버리지 측정 명령어: `cargo llvm-cov --html --branch`
- HTML 리포트 위치: `target/llvm-cov/html/index.html`
- 브랜치 커버리지 포함: `--branch` 플래그 사용

### 2.2 테스트 파일 구조

**기존 테스트 파일 확장:**
```
tests/
├── storage_tests.rs       (기존: 6개) → 목표: 100%
├── model_tests.rs         (기존: 24개) → 목표: 100%
├── update_tests.rs        (기존: 일부) → 목표: 100%
├── view_tests.rs          (기존: 9개) → 목표: 100%
├── markdown_tests.rs      (기존: 11개) → 목표: 100%
└── integration_tests.rs   (신규 생성) → main.rs 커버리지
```

### 2.3 새로운 테스트 모듈

**integration_tests.rs (신규):**
- main.rs의 이벤트 루프 테스트
- 전체 애플리케이션 워크플로우 테스트
- 키보드 입력 시뮬레이션
- 화면 전환 시나리오

**추가할 테스트 카테고리:**
- 에러 케이스 (현재 부족)
- 경계값 테스트 (edge cases)
- 브랜치 커버리지 (if/match의 모든 분기)
- private 함수 (간접적으로 테스트)

### 2.4 테스트 헬퍼 유틸리티

**공통 헬퍼 함수:**
```rust
// tests/common/mod.rs (신규 생성)
- setup_test_model() → 테스트용 Model 생성
- create_temp_storage() → 임시 Storage
- simulate_key_sequence() → 키 입력 시뮬레이션
- assert_coverage_increased() → 커버리지 검증
```

---

## 3. 데이터 흐름

### 3.1 커버리지 측정 워크플로우

```
[1단계: 초기 측정]
cargo install cargo-llvm-cov
    ↓
cargo llvm-cov --html --branch
    ↓
HTML 리포트 생성 (target/llvm-cov/html/)
    ↓
현재 커버리지 확인 (예: 65%)

[2단계: 모듈별 개선 - storage.rs]
리포트에서 storage.rs 누락 라인 식별
    ↓
누락된 함수/브랜치 목록 작성
    ↓
BDD 형식으로 테스트 시나리오 작성
    ↓
tests/storage_tests.rs에 테스트 추가
    ↓
cargo test 실행 (테스트 통과 확인)
    ↓
cargo llvm-cov --html --branch (재측정)
    ↓
storage.rs 100% 달성 확인

[3단계: 다음 모듈로 반복]
model.rs → update.rs → view.rs → markdown.rs → main.rs
(각 모듈마다 2단계 과정 반복)

[4단계: 최종 검증]
cargo llvm-cov --html --branch
    ↓
전체 프로젝트 100% 확인
    ↓
커버리지 리포트 커밋
```

### 3.2 테스트 작성 프로세스

```
[누락 코드 발견]
HTML 리포트의 빨간색/노란색 라인
    ↓
[BDD 명세 작성]
Given-When-Then 형식으로 시나리오 작성
예: "Given 빈 문서, When 문자 삽입, Then 커서 이동"
    ↓
[TDD 사이클]
1. Red: 실패하는 테스트 작성
2. Green: 최소한의 코드로 통과
3. Refactor: 리팩토링 (이미 구현됨)
    ↓
[검증]
해당 라인이 초록색으로 변경되었는지 확인
```

### 3.3 데이터 추적

**커버리지 진행 상황 추적:**
- 각 커밋마다 커버리지 백분율 기록
- 모듈별 커버리지 표 작성
- 목표: 모든 모듈 100%

---

## 4. 테스트 전략

### 4.1 모듈별 테스트 전략

#### storage.rs (우선순위: 1)
- 파일 I/O 에러 케이스 추가
- `parse_filename()` private 함수 간접 테스트
- 잘못된 파일명 처리
- 디스크 공간 부족 시뮬레이션 (선택적)

#### model.rs (우선순위: 2)
- `days_in_month()` 함수 테스트 (현재 누락)
- `char_idx_to_byte_idx()`, `byte_idx_to_char_idx()` 엣지 케이스
- `restore_snapshot()` private 함수 간접 테스트
- 히스토리 크기 제한 (MAX_HISTORY) 테스트
- 모든 EditorState 메서드의 경계값 테스트

#### update.rs (우선순위: 3)
- 모든 Msg variant 처리 테스트
- `update_selection_on_move()` helper 함수
- `ensure_selection_for_edit()` helper 함수
- `paste_clipboard()` 모든 분기 (줄 단위/문자 단위)
- 각 화면(Calendar/Editor)별 메시지 처리
- 서브모드(Goto/Space/Search) 전환 테스트

#### view.rs (우선순위: 4)
- 모든 렌더링 함수 테스트
- 다양한 화면 크기 시뮬레이션
- 스타일 적용 확인
- 선택 영역 하이라이트
- 검색 매치 하이라이트

#### markdown.rs (우선순위: 5)
- `create_skin()` 함수 간접 테스트
- `convert_fmt_text_to_ratatui()` 엣지 케이스
- `to_ratatui_color()` 모든 색상 변환
- 파싱 실패 fallback 테스트
- 복잡한 마크다운 조합

#### main.rs (우선순위: 6)
- 통합 테스트로 간접 커버
- 이벤트 루프 시뮬레이션
- 전체 사용자 시나리오 테스트

### 4.2 테스트 작성 원칙

**BDD 형식 준수:**
```rust
#[test]
fn test_scenario_name() {
    // Given: 초기 상태 설정
    let mut state = setup_initial_state();

    // When: 동작 실행
    perform_action(&mut state);

    // Then: 결과 검증
    assert_eq!(state.expected, actual);
}
```

**TDD 사이클:**
1. 테스트 먼저 작성 (이미 구현된 코드지만 테스트는 나중)
2. 테스트 실행하여 통과 확인
3. 커버리지 확인

**테스트 명명 규칙:**
- `test_<기능>_<조건>_<예상결과>`
- 예: `test_insert_char_at_line_end_moves_cursor`

### 4.3 커버리지 목표

- **라인 커버리지:** 100%
- **브랜치 커버리지:** 100% (모든 if/match 분기)
- **함수 커버리지:** 100% (모든 public/private 함수)

---

## 5. 에러 처리

### 5.1 테스트 실패 시 대응

**테스트 실패 패턴:**
```
테스트 실패 발견
    ↓
실패 원인 분석
    ├─ 테스트 로직 오류 → 테스트 수정
    ├─ 구현 버그 발견 → 버그 수정 후 테스트
    └─ 엣지 케이스 → 추가 테스트 작성
```

**디버깅 전략:**
- `cargo test -- --nocapture` 로 상세 출력
- `#[ignore]` 태그로 문제 테스트 격리
- 작은 단위로 테스트 분할

### 5.2 커버리지 측정 오류

**cargo-llvm-cov 설치 실패:**
- rustup update로 툴체인 업데이트
- llvm-tools-preview 컴포넌트 수동 설치
- 대안: tarpaulin 사용

**커버리지 리포트 생성 실패:**
- 이전 빌드 아티팩트 삭제: `cargo clean`
- 캐시 클리어 후 재시도
- 개별 모듈 측정: `cargo llvm-cov --lib --html`

**100% 달성 불가능한 경우:**
- `#[cfg(test)]` 블록은 제외
- Unreachable 코드 식별 및 제거
- Dead code 제거 (`#[allow(dead_code)]` 확인)

### 5.3 통합 테스트 문제

**main.rs 테스트 어려움:**
- 이벤트 루프를 모킹 가능한 구조로 리팩토링 고려
- 대안: main.rs를 얇게 유지하고 로직을 lib.rs로 이동
- 최후: 통합 테스트로 간접 커버 (일부 라인은 미커버 허용 가능)

**외부 의존성 모킹:**
- crossterm 이벤트를 테스트용 구조체로 래핑
- ratatui 렌더링은 메모리 버퍼로 캡처
- tempfile 사용하여 파일 시스템 격리

### 5.4 품질 보증

**최종 검증 체크리스트:**
- [ ] 모든 테스트 통과 (`cargo test`)
- [ ] 전체 커버리지 100% (`cargo llvm-cov --html --branch`)
- [ ] Clippy 경고 없음 (`cargo clippy`)
- [ ] 포맷팅 준수 (`cargo fmt --check`)
- [ ] 문서화 완료 (커버리지 리포트 위치, 실행 방법)

**지속적 품질 유지:**
- CI/CD에 커버리지 체크 추가 (선택적)
- pre-commit hook으로 테스트 자동 실행
- 커버리지 하락 방지 정책

---

## 6. 구현 순서

### Phase 1: 환경 설정
1. cargo-llvm-cov 설치
2. 초기 커버리지 측정
3. HTML 리포트 분석

### Phase 2: 모듈별 개선
1. storage.rs → 100%
2. model.rs → 100%
3. update.rs → 100%
4. view.rs → 100%
5. markdown.rs → 100%

### Phase 3: 통합 테스트
1. integration_tests.rs 생성
2. main.rs 커버리지 향상
3. 전체 시나리오 테스트

### Phase 4: 최종 검증
1. 전체 커버리지 100% 확인
2. 문서화
3. 커밋 및 리뷰

---

## 7. 성공 기준

✅ 모든 소스 파일 라인 커버리지 100%
✅ 브랜치 커버리지 100%
✅ 모든 테스트 통과
✅ Clippy 경고 없음
✅ 커버리지 리포트 문서화

---

## 8. 참고 자료

- [cargo-llvm-cov 문서](https://github.com/taiki-e/cargo-llvm-cov)
- [Rust 테스트 베스트 프랙티스](https://doc.rust-lang.org/book/ch11-00-testing.html)
- 프로젝트 CLAUDE.md (TDD/BDD 요구사항)
