---
name: tdd-workflow
description: >
  요구사항 하나로 PRD -> SPEC -> TEST -> IMPL -> RUN 전체 TDD 워크플로우를 자동 실행하는 스킬.
  사용자가 기능 요구사항, 새로운 기능, 요구사항 추가, PRD 작성, 스펙 작성, TDD, 테스트 주도 개발,
  기능 구현, "/tdd", "/implement", "/feature" 등을 언급하면 반드시 이 스킬을 사용하라.
  요구사항 하나를 입력하면 PRD ID가 부여된 PRD를 docs/prd/ 에 추가하고,
  SPEC ID가 부여된 기술 명세서를 docs/spec/ 에 생성하고,
  TDD 방식으로 테스트 코드를 먼저 작성한 뒤, 기능을 구현하고, 테스트를 자동 실행한다.
  모든 산출물 간에 정방향(PRD->SPEC->TEST->CODE) 및 역방향(CODE->TEST->SPEC->PRD) 추적성을 보장한다.
  단순한 버그 수정이나 리팩토링에는 사용하지 않는다.
  추적성 매트릭스 생성, 추적 검증, "/trace", "/traceability" 를 언급해도 이 스킬을 사용하라.
---

# TDD Workflow Skill

하나의 요구사항을 입력받아 PRD -> SPEC -> TEST -> IMPL -> RUN 전체 파이프라인을 자동으로 실행한다.
모든 산출물 사이에 양방향 추적성(Traceability)을 보장하는 것이 이 스킬의 핵심 원칙이다.

## 추적성 원칙

이 스킬이 생성하는 모든 산출물은 다음 추적 체인을 형성한다:

```
PRD (FR-n) <-> SPEC (SPEC-xxx) <-> TEST (TC-n) <-> CODE (함수/클래스)
```

정방향 추적: "이 요구사항은 어디에 구현되어 있는가?"
역방향 추적: "이 코드는 왜 존재하는가? 어떤 요구사항에서 비롯되었는가?"

추적성을 보장하기 위해 다음 규칙을 반드시 준수한다:

1. **모든 산출물에 추적 헤더를 삽입한다.** 문서에는 메타데이터 섹션, 코드에는 구조화된 주석 블록.
2. **모든 참조는 양방향이다.** PRD가 SPEC을 참조하면 SPEC도 PRD를 참조한다. SPEC이 TC를 정의하면 TC 코드가 SPEC을 참조한다.
3. **추적 단위는 기능 요구사항(FR)이다.** PRD의 FR-n 하나가 추적의 시작점이며, 이것이 SPEC, TC, 구현 함수까지 일관되게 연결된다.
4. **추적성 매트릭스를 자동 생성한다.** Phase 6에서 docs/traceability-matrix.md 를 생성하여 전체 추적 현황을 한눈에 보여준다.
5. **추적 태그 형식은 `@trace` 를 사용한다.** 코드 내 추적 주석은 grep, 정규식으로 기계적 파싱이 가능해야 한다.

## 추적 태그 규약

모든 코드 파일(테스트, 구현)에 삽입하는 추적 주석의 형식이다. 언어에 관계없이 동일한 구조를 사용한다.

### 테스트 파일 추적 헤더

파일 최상단에 반드시 삽입한다:

```python
# =============================================================================
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: test
# =============================================================================
```

```typescript
// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2
// @trace file-type: test
// =============================================================================
```

### 테스트 함수별 추적 태그

각 테스트 함수에도 개별 추적을 명시한다:

```python
def test_tc1_login_success():
    """
    @trace TC: SPEC-001/TC-1
    @trace FR: PRD-001/FR-1
    @trace scenario: 정상 로그인 시 JWT 토큰 반환
    """
    # ...
```

```typescript
/**
 * @trace TC: SPEC-001/TC-1
 * @trace FR: PRD-001/FR-1
 * @trace scenario: 정상 로그인 시 JWT 토큰 반환
 */
test('TC-1: login success returns JWT', () => {
    // ...
});
```

### 구현 파일 추적 헤더

```python
# =============================================================================
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: impl
# =============================================================================
```

### 구현 함수/클래스별 추적 태그

```python
def authenticate_user(email: str, password: str) -> Token:
    """사용자를 인증하고 JWT 토큰을 반환한다.

    @trace SPEC: SPEC-001
    @trace TC: SPEC-001/TC-1, SPEC-001/TC-2, SPEC-001/TC-3
    @trace FR: PRD-001/FR-1
    """
    # ...
```

```typescript
/**
 * 사용자를 인증하고 JWT 토큰을 반환한다.
 *
 * @trace SPEC: SPEC-001
 * @trace TC: SPEC-001/TC-1, SPEC-001/TC-2, SPEC-001/TC-3
 * @trace FR: PRD-001/FR-1
 */
export function authenticateUser(email: string, password: string): Token {
    // ...
}
```

## 디렉토리 규약

```
project-root/
├── docs/
│   ├── prd/                        # PRD 문서들
│   │   ├── PRD-001.md
│   │   └── ...
│   ├── spec/                       # SPEC 명세서들
│   │   ├── SPEC-001.md
│   │   └── ...
│   ├── registry.json               # ID 레지스트리 + 추적 매핑
│   └── traceability-matrix.md      # 추적성 매트릭스 (자동 생성)
├── tests/                          # 테스트 코드
│   ├── spec-001/
│   │   └── ...
│   └── spec-002/
│       └── ...
└── src/                            # 구현 코드
```

## Phase 0: 초기화

프로젝트에 docs/ 디렉토리가 없으면 자동 생성한다.

```bash
mkdir -p docs/prd docs/spec
```

registry.json 이 없으면 초기 파일을 생성한다:

```json
{
  "last_prd_id": 0,
  "last_spec_id": 0,
  "entries": [],
  "trace_map": {}
}
```

`trace_map` 은 추적성 색인이다. Phase별로 자동 갱신된다. 구조는 다음과 같다:

```json
{
  "trace_map": {
    "PRD-001": {
      "fr": {
        "FR-1": {
          "title": "이메일/비밀번호 로그인",
          "specs": ["SPEC-001"],
          "test_cases": ["SPEC-001/TC-1", "SPEC-001/TC-2", "SPEC-001/TC-3"],
          "test_files": ["tests/spec-001/test_auth.py"],
          "impl_files": ["src/auth/login.py"],
          "impl_symbols": ["authenticate_user", "validate_password"],
          "status": "passed"
        },
        "FR-2": {
          "title": "JWT 토큰 갱신",
          "specs": ["SPEC-002"],
          "test_cases": ["SPEC-002/TC-1", "SPEC-002/TC-2"],
          "test_files": ["tests/spec-002/test_refresh.py"],
          "impl_files": ["src/auth/refresh.py"],
          "impl_symbols": ["refresh_token"],
          "status": "pending"
        }
      }
    }
  }
}
```

테스트 프레임워크를 감지한다. package.json, pytest.ini, Cargo.toml, go.mod 등을 확인하여 프로젝트의 언어와 테스트 러너를 파악한다.

## Phase 1: PRD 작성

사용자의 요구사항을 받아 PRD 문서를 작성한다.

1. registry.json 에서 `last_prd_id` 를 읽고 +1 하여 새 PRD ID를 생성한다.
   - 형식: `PRD-001`, `PRD-002`, ... (3자리 0-패딩)

2. 사용자에게 다음을 확인한다 (맥락에서 유추 가능하면 확인 생략):
   - 기능의 목적과 배경
   - 대상 사용자
   - 성공 기준
   - 제약 조건

3. PRD 문서를 `docs/prd/PRD-{ID}.md` 에 작성한다:

```markdown
# PRD-{ID}: {제목}

## 메타데이터
- PRD ID: PRD-{ID}
- 작성일: {YYYY-MM-DD}
- 상태: Draft

## 추적 정보
| FR ID | 요구사항 | 관련 SPEC | 상태 |
|-------|---------|----------|------|
| FR-1  | {요구사항 1} | (Phase 2에서 자동 채움) | Draft |
| FR-2  | {요구사항 2} | (Phase 2에서 자동 채움) | Draft |

## 배경 및 목적
{왜 이 기능이 필요한지}

## 대상 사용자
{누구를 위한 기능인지}

## 요구사항
### 기능 요구사항 (Functional)
- FR-1: {기능 요구사항 1}
- FR-2: {기능 요구사항 2}

### 비기능 요구사항 (Non-Functional)
- NFR-1: {성능, 보안 등}

## 성공 기준
- {측정 가능한 기준}

## 범위 외 (Out of Scope)
- {이번에 하지 않는 것}

## 제약 조건
- {기술적, 일정, 리소스 제약}
```

핵심: **요구사항 섹션의 모든 기능 요구사항에 FR-n 번호를 부여한다.** 이 FR 번호가 이후 모든 추적의 시작점이다. 추적 정보 테이블은 Phase 2 이후에 자동으로 갱신된다.

4. registry.json 을 업데이트한다:
   - `last_prd_id` 증가
   - entries 배열에 PRD 항목 추가
   - `trace_map` 에 PRD 항목과 FR 항목 초기화:

```json
{
  "trace_map": {
    "PRD-001": {
      "fr": {
        "FR-1": {
          "title": "기능 요구사항 1",
          "specs": [],
          "test_cases": [],
          "test_files": [],
          "impl_files": [],
          "impl_symbols": [],
          "status": "draft"
        }
      }
    }
  }
}
```

5. 사용자에게 PRD 요약을 보여주고 확인을 받는다. 수정 요청이 있으면 반영한다.

## Phase 2: SPEC 작성

PRD의 기능 요구사항을 기술 명세서로 변환한다. 이 단계에서 FR과 SPEC 사이의 매핑을 확립한다.

1. registry.json 에서 `last_spec_id` 를 읽고 +1 하여 새 SPEC ID를 생성한다.
   - 형식: `SPEC-001`, `SPEC-002`, ... (3자리 0-패딩)
   - 하나의 PRD에서 여러 SPEC이 파생될 수 있다.
   - 일반적으로 하나의 독립적인 기능 단위 = 하나의 SPEC
   - **하나의 SPEC이 커버하는 FR 목록을 명확히 정의한다.**

2. SPEC 문서를 `docs/spec/SPEC-{ID}.md` 에 작성한다:

```markdown
# SPEC-{ID}: {기능명}

## 메타데이터
- SPEC ID: SPEC-{ID}
- PRD: PRD-{PRD_ID}
- 작성일: {YYYY-MM-DD}
- 상태: Draft

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-{PRD_ID} | FR-1 | {요구사항 1} |
| PRD-{PRD_ID} | FR-2 | {요구사항 2} |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | {시나리오} | FR-1 | (Phase 3에서 자동 채움) | Pending |
| TC-2  | {시나리오} | FR-1 | (Phase 3에서 자동 채움) | Pending |
| TC-3  | {시나리오} | FR-2 | (Phase 3에서 자동 채움) | Pending |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| (Phase 4에서 자동 채움) | | |

## 개요
{이 SPEC이 구현하는 기능에 대한 한 줄 설명}

## 기술 설계
### 아키텍처
{컴포넌트 구조, 데이터 흐름}

### API / 인터페이스
{함수 시그니처, 엔드포인트, 입출력 타입}

### 데이터 모델
{필요한 경우 데이터 구조 정의}

## 테스트 시나리오
다음 테스트 케이스를 TDD 단계에서 구현한다.
**각 TC는 반드시 하나 이상의 FR과 매핑되어야 한다.** FR 매핑이 없는 TC는 작성하지 않는다.

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | {정상 케이스} | {입력값} | {기대값} | unit | FR-1 |
| TC-2 | {경계 케이스} | {입력값} | {기대값} | unit | FR-1 |
| TC-3 | {에러 케이스} | {입력값} | {에러} | unit | FR-2 |

## 구현 가이드
- 파일 위치: {src/...}
- 의존성: {필요한 라이브러리나 모듈}
- 주의사항: {함정, 엣지케이스}

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] 코드 리뷰 완료
```

핵심: **테스트 시나리오 테이블에 "검증 대상 FR" 열을 반드시 포함한다.** 이것이 SPEC 수준에서 FR->TC 매핑의 근거가 된다. 모든 FR에 대해 최소 1개의 TC가 존재해야 한다. FR이 TC로 커버되지 않으면 누락된 TC를 추가한다.

3. registry.json 을 업데이트한다:
   - `last_spec_id` 증가
   - entries 배열에 SPEC 항목 추가
   - **`trace_map` 에서 해당 FR 항목의 `specs` 배열에 SPEC ID 추가**
   - **`trace_map` 에서 해당 FR 항목의 `test_cases` 배열에 TC ID 추가**

4. PRD 문서의 추적 정보 테이블을 업데이트한다:
   - 각 FR의 "관련 SPEC" 열에 SPEC ID를 채운다.

5. **추적 검증**: 이 시점에서 다음을 확인한다:
   - 모든 FR에 대해 최소 1개의 SPEC이 매핑되어 있는가?
   - 모든 FR에 대해 최소 1개의 TC가 정의되어 있는가?
   - 누락이 있으면 사용자에게 경고하고 보완한다.

## Phase 3: TDD 테스트 코드 작성

SPEC의 테스트 시나리오를 기반으로 테스트 코드를 먼저 작성한다. 이 단계에서는 구현 코드를 절대 작성하지 않는다.

1. Phase 0에서 감지한 테스트 프레임워크에 맞춰 테스트 파일을 생성한다.
   - 테스트 디렉토리: `tests/spec-{id}/` 또는 프로젝트 관례에 따른다.

2. **추적 헤더 삽입** (필수): 테스트 파일 최상단에 반드시 추적 헤더를 삽입한다.

```python
# =============================================================================
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: test
# =============================================================================
```

3. **테스트 함수별 추적 태그** (필수): 각 테스트 함수에 @trace 태그를 삽입한다.

```python
def test_tc1_login_success():
    """
    @trace TC: SPEC-001/TC-1
    @trace FR: PRD-001/FR-1
    @trace scenario: 정상 로그인 시 JWT 토큰 반환
    """
    result = authenticate_user("user@test.com", "valid_password")
    assert result.token is not None
    assert result.token_type == "Bearer"


def test_tc2_login_invalid_password():
    """
    @trace TC: SPEC-001/TC-2
    @trace FR: PRD-001/FR-1
    @trace scenario: 잘못된 비밀번호로 401 반환
    """
    with pytest.raises(AuthenticationError) as exc:
        authenticate_user("user@test.com", "wrong_password")
    assert exc.value.status_code == 401
```

4. 테스트를 실행하여 모든 테스트가 실패(RED)하는 것을 확인한다.

5. **추적 갱신**: SPEC 문서의 역방향 추적 테이블에서 각 TC의 "테스트 파일" 열을 실제 파일 경로로 채운다.

6. registry.json 의 `trace_map` 에서 해당 FR 항목의 `test_files` 배열에 테스트 파일 경로를 추가한다.

## Phase 4: 기능 구현

SPEC과 테스트 코드를 기반으로 기능을 구현한다.

1. SPEC 문서의 구현 가이드를 참조하여 코드를 작성한다.

2. **추적 헤더 삽입** (필수): 구현 파일 최상단에 반드시 추적 헤더를 삽입한다.

```python
# =============================================================================
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: impl
# =============================================================================
```

3. **함수/클래스별 추적 태그** (필수): 구현하는 각 함수 또는 클래스에 @trace 태그를 삽입한다.

```python
def authenticate_user(email: str, password: str) -> AuthResult:
    """사용자를 인증하고 JWT 토큰을 반환한다.

    @trace SPEC: SPEC-001
    @trace TC: SPEC-001/TC-1, SPEC-001/TC-2, SPEC-001/TC-3
    @trace FR: PRD-001/FR-1
    """
    # ...
```

이 태그는 "이 함수가 왜 존재하는가"를 코드만 보고 즉시 알 수 있게 한다. 어떤 요구사항에서 비롯되었고, 어떤 테스트가 이 함수를 검증하는지를 코드 안에서 확인 가능하다.

4. 테스트를 하나씩 통과시키는 방식으로 구현한다 (GREEN).
5. 불필요한 중복이 있으면 리팩토링한다 (REFACTOR). 리팩토링 시에도 @trace 태그를 유지한다.

6. **추적 갱신**:
   - SPEC 문서의 구현 추적 테이블에 구현 파일 경로와 심볼(함수/클래스)명을 채운다.
   - registry.json 의 `trace_map` 에서 해당 FR 항목의 `impl_files`와 `impl_symbols` 를 채운다.

## Phase 5: 테스트 실행 및 검증

구현이 완료되면 전체 테스트를 실행하고, **추적성 무결성을 검증한다.**

1. 해당 SPEC의 테스트를 실행한다.
2. 결과를 파싱하여 사용자에게 보고한다:

```
=== TDD 결과 보고 ===
SPEC: SPEC-001
PRD:  PRD-001

테스트 결과:
  TC-1 (FR-1) 정상 케이스      PASS
  TC-2 (FR-1) 경계 케이스      PASS
  TC-3 (FR-2) 에러 케이스      PASS

통과: 3/3 (100%)
상태: GREEN
```

3. **추적성 무결성 검증** (필수): 다음 항목을 확인한다.

```
=== 추적성 검증 ===

[정방향] PRD-001 요구사항 커버리지:
  FR-1 "이메일/비밀번호 로그인"
    -> SPEC: SPEC-001                    OK
    -> TC:   SPEC-001/TC-1, TC-2         OK (2개)
    -> IMPL: src/auth/login.py           OK
       authenticate_user()               OK
       validate_password()               OK

  FR-2 "JWT 토큰 갱신"
    -> SPEC: SPEC-001                    OK
    -> TC:   SPEC-001/TC-3               OK (1개)
    -> IMPL: src/auth/login.py           OK
       generate_jwt()                    OK

[역방향] 코드 -> 요구사항 역추적:
  src/auth/login.py
    authenticate_user()  -> SPEC-001/TC-1,TC-2 -> FR-1 -> PRD-001   OK
    validate_password()  -> SPEC-001/TC-2      -> FR-1 -> PRD-001   OK
    generate_jwt()       -> SPEC-001/TC-3      -> FR-2 -> PRD-001   OK

[누락 검사]
  FR 없는 TC:        없음   OK
  TC 없는 FR:        없음   OK
  구현 없는 TC:      없음   OK
  추적태그 없는 함수: 없음   OK

추적성: 완전 (COMPLETE)
```

추적성 검증에서 문제가 발견되면:
- **FR 없는 TC**: 해당 TC에 FR 매핑 추가
- **TC 없는 FR**: 해당 FR에 대한 TC 추가 작성 후 구현
- **구현 없는 TC**: 해당 TC를 통과시키는 구현 추가
- **추적태그 없는 함수**: 해당 함수에 @trace 태그 추가

4. 모든 테스트가 통과하고 추적성이 완전하면:
   - SPEC 문서의 상태를 `Implemented` 로 변경
   - registry.json 의 test_status 를 `passed`, 각 FR의 status를 `passed` 로 변경
   - PRD의 모든 FR이 완료되면 PRD 상태도 `Implemented` 로 변경

5. 실패한 테스트가 있으면:
   - 실패 원인을 분석하고 수정 시도
   - 최대 3회 재시도 후에도 실패하면 사용자에게 보고하고 판단을 맡긴다

## Phase 6: 추적성 매트릭스 생성 및 완료 보고

모든 단계가 끝나면 추적성 매트릭스를 생성하고 최종 보고를 출력한다.

### 추적성 매트릭스 생성

`docs/traceability-matrix.md` 를 자동 생성(또는 갱신)한다:

```markdown
# 추적성 매트릭스

최종 갱신: {YYYY-MM-DD HH:MM}

## 정방향 추적 (요구사항 -> 구현)

| PRD | FR ID | FR 제목 | SPEC | TC | 테스트 파일 | 구현 파일 | 구현 심볼 | 테스트 상태 |
|-----|-------|--------|------|-----|-----------|----------|----------|-----------|
| PRD-001 | FR-1 | 이메일 로그인 | SPEC-001 | TC-1, TC-2 | tests/spec-001/test_auth.py | src/auth/login.py | authenticate_user, validate_password | PASS |
| PRD-001 | FR-2 | JWT 갱신 | SPEC-001 | TC-3 | tests/spec-001/test_auth.py | src/auth/login.py | generate_jwt | PASS |

## 역방향 추적 (구현 -> 요구사항)

| 구현 파일 | 심볼 | SPEC | TC | FR | PRD | 테스트 상태 |
|----------|------|------|-----|-----|-----|-----------|
| src/auth/login.py | authenticate_user | SPEC-001 | TC-1, TC-2 | PRD-001/FR-1 | PRD-001 | PASS |
| src/auth/login.py | validate_password | SPEC-001 | TC-2 | PRD-001/FR-1 | PRD-001 | PASS |
| src/auth/login.py | generate_jwt | SPEC-001 | TC-3 | PRD-001/FR-2 | PRD-001 | PASS |

## 커버리지 요약

| PRD | 전체 FR | 커버된 FR | SPEC 수 | TC 수 | 통과 | 실패 | 커버리지 |
|-----|--------|----------|--------|-------|------|------|---------|
| PRD-001 | 2 | 2 | 1 | 3 | 3 | 0 | 100% |

## 미추적 항목 (경고)

없음
```

이 매트릭스는 registry.json 의 `trace_map` 으로부터 자동 생성된다.

### 최종 보고

```
=== TDD Workflow 완료 ===
PRD:   PRD-001 "기능 제목"
SPEC:  SPEC-001 "세부 기능명"

생성된 파일:
  docs/prd/PRD-001.md
  docs/spec/SPEC-001.md
  docs/traceability-matrix.md (갱신)
  tests/spec-001/test_auth.py
  src/auth/login.py

추적성:
  정방향: FR 2개 -> SPEC 1개 -> TC 3개 -> IMPL 3개 함수   완전
  역방향: IMPL 3개 함수 -> TC 3개 -> FR 2개 -> PRD 1개     완전

테스트: 3/3 통과 (100%)
상태: 완료
```

## 에러 처리

- registry.json 읽기/쓰기 실패 시: 파일을 재생성하되, 기존 docs/prd/, docs/spec/ 디렉토리를 스캔하여 ID를 복원한다. trace_map은 코드 파일의 @trace 태그를 grep하여 재구성한다.
- 테스트 프레임워크 미설치 시: 사용자에게 설치 방법을 안내하고, 설치 후 계속 진행한다.
- git 충돌 시: 사용자에게 알리고 수동 해결을 요청한다.
- **추적 태그 파싱 실패 시**: @trace 형식이 깨진 파일을 목록으로 보여주고 수정을 제안한다.

## 부분 실행

사용자가 특정 Phase만 실행하고 싶을 수 있다:
- "PRD만 작성해줘" -> Phase 0-1만 실행
- "SPEC-001 구현해줘" -> 기존 SPEC을 읽어 Phase 3-5 실행, 추적 태그 포함
- "SPEC-001 테스트만 실행해줘" -> Phase 5만 실행
- "PRD-001에 기능 추가해줘" -> 기존 PRD에 FR 추가 후 Phase 2부터 실행, trace_map 갱신
- "추적성 매트릭스 갱신해줘" -> Phase 6의 매트릭스 생성만 실행
- "추적성 검증해줘" -> Phase 5의 추적성 무결성 검증만 실행

## 추적성 검증 명령

사용자가 "/trace" 또는 "추적성 검증" 을 요청하면, 현재 프로젝트 전체에 대해 다음을 수행한다:

1. docs/prd/ 의 모든 PRD에서 FR 목록을 추출한다.
2. docs/spec/ 의 모든 SPEC에서 TC와 FR 매핑을 추출한다.
3. 테스트 파일에서 @trace 태그를 grep하여 TC->FR 매핑을 추출한다.
4. 구현 파일에서 @trace 태그를 grep하여 함수->SPEC->FR 매핑을 추출한다.
5. 정방향/역방향 추적 체인의 빈 항목을 찾아 보고한다.
6. docs/traceability-matrix.md 를 갱신한다.

grep 명령 예시:
```bash
# 프로젝트 전체에서 @trace 태그 추출
grep -rn "@trace" tests/ src/ --include="*.py" --include="*.ts" --include="*.js" --include="*.go" --include="*.rs"

# FR 매핑만 추출
grep -rn "@trace FR:" tests/ src/

# SPEC 매핑만 추출
grep -rn "@trace SPEC:" tests/ src/

# TC 매핑만 추출
grep -rn "@trace TC:" tests/ src/
```

## 다중 SPEC 처리

하나의 PRD가 여러 SPEC으로 분할될 경우:
1. 각 SPEC을 순차적으로 처리한다 (SPEC별로 Phase 2-5 반복).
2. 각 SPEC 완료 시 trace_map을 즉시 갱신한다.
3. 모든 SPEC이 완료된 후 통합 테스트를 실행한다.
4. PRD 수준의 추적성 검증을 실행하여 모든 FR이 커버되었는지 최종 확인한다.
5. 추적성 매트릭스를 생성하고 PRD 수준의 완료 보고를 작성한다.

## 코드 변경 시 추적성 유지

기존 코드를 수정하는 경우에도 추적성을 유지해야 한다:
- 함수를 이동하면: @trace 태그의 파일 경로를 갱신하고, SPEC 문서의 구현 추적 테이블도 갱신한다.
- 함수를 분할하면: 분할된 각 함수에 원래 함수의 @trace 태그를 복사한다.
- 함수를 삭제하면: 해당 함수의 TC가 여전히 다른 함수에 의해 커버되는지 확인한다.
- TC를 추가하면: 반드시 FR 매핑을 포함한다.
- FR을 변경하면: 관련 SPEC, TC, 구현 코드의 @trace 태그를 모두 갱신한다.
