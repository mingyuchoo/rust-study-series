# TDD Workflow Skill

하나의 요구사항을 입력하면 PRD -> SPEC -> TEST -> IMPL -> RUN 전체 TDD 파이프라인을 자동 실행하는 Claude Code 스킬.
모든 산출물 사이에 정방향/역방향 양방향 추적성(Traceability)을 보장한다.

## 핵심 개념: 양방향 추적성

이 스킬이 만드는 모든 산출물은 다음 추적 체인을 형성한다:

```
PRD (FR-n) <-> SPEC (SPEC-xxx) <-> TEST (TC-n) <-> CODE (함수/클래스)
```

- 정방향: "이 요구사항은 어디에 구현되어 있는가?"
- 역방향: "이 코드는 왜 존재하는가?"

### 추적성을 보장하는 3가지 메커니즘

1. **문서 간 양방향 참조 테이블**: PRD 문서에 FR->SPEC 매핑, SPEC 문서에 FR->TC 및 TC->파일 매핑이 테이블로 유지된다.

2. **코드 내 `@trace` 태그**: 모든 테스트/구현 파일에 구조화된 추적 주석이 삽입되어, `grep -rn "@trace"` 한 줄로 전체 추적 관계를 추출할 수 있다.

3. **`registry.json`의 `trace_map`**: FR 단위로 SPEC, TC, 테스트 파일, 구현 파일, 구현 심볼까지 전체 매핑을 기계적으로 추적하는 색인 구조.

## 설치 방법

### 글로벌 설치 (모든 프로젝트에서 사용)

```bash
cp -r tdd-workflow-skill ~/.claude/skills/tdd-workflow
```

### 프로젝트별 설치

```bash
mkdir -p .claude/skills
cp -r tdd-workflow-skill .claude/skills/tdd-workflow
```

### CLAUDE.md 에 등록

```markdown
## TDD Workflow
기능 요구사항을 구현할 때 /tdd-workflow 스킬을 사용한다.
요구사항 하나를 입력하면 PRD -> SPEC -> TEST -> IMPL -> RUN 을 자동 실행한다.
추적성 검증은 /trace 로 실행한다.
```

## 사용 예시

### 전체 파이프라인

```
You: 사용자 로그인 기능을 JWT 기반으로 구현해줘

Claude: [Phase 1] PRD-001 작성 - FR-1: 이메일/비밀번호 로그인, FR-2: JWT 갱신
        [Phase 2] SPEC-001 작성 - TC-1(FR-1), TC-2(FR-1), TC-3(FR-2)
        [Phase 3] 테스트 코드 작성 - @trace 태그 삽입, RED 확인
        [Phase 4] 구현 코드 작성 - @trace 태그 삽입, GREEN 확인
        [Phase 5] 추적성 검증
          정방향: FR-1 -> SPEC-001 -> TC-1,TC-2 -> authenticate_user()  OK
          역방향: authenticate_user() -> TC-1,TC-2 -> FR-1 -> PRD-001   OK
        [Phase 6] 추적성 매트릭스 생성, 완료 보고
```

### 추적성 검증

```
You: 추적성 검증해줘

Claude: === 추적성 검증 ===
        [정방향] PRD-001 FR-1 -> SPEC-001 -> TC-1,TC-2 -> login.py  OK
        [역방향] login.py:authenticate_user -> TC-1,TC-2 -> FR-1    OK
        [누락 검사] 없음
        추적성: 완전 (COMPLETE)
```

## `@trace` 태그 예시

```python
# 테스트 파일 상단
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: test

def test_tc1_login_success():
    """
    @trace TC: SPEC-001/TC-1
    @trace FR: PRD-001/FR-1
    @trace scenario: 정상 로그인 시 JWT 토큰 반환
    """
    ...

# 구현 파일
# @trace SPEC-001
# @trace PRD: PRD-001
# @trace FR: FR-1, FR-2
# @trace file-type: impl

def authenticate_user(email, password):
    """
    @trace SPEC: SPEC-001
    @trace TC: SPEC-001/TC-1, SPEC-001/TC-2, SPEC-001/TC-3
    @trace FR: PRD-001/FR-1
    """
    ...
```

## 파일 구조

```
tdd-workflow-skill/
├── SKILL.md                  # 메인 스킬 정의
├── README.md                 # 이 파일
├── scripts/
│   ├── registry.py           # ID 레지스트리 + trace_map 관리
│   └── verify_trace.py       # 추적성 검증 + 매트릭스 생성
├── references/
│   └── customization.md      # 커스터마이징 가이드
└── templates/                # (선택) 커스텀 PRD/SPEC 템플릿
```

## 스크립트 사용법

### verify_trace.py

```bash
python scripts/verify_trace.py              # 추적성 검증
python scripts/verify_trace.py --matrix     # 추적성 매트릭스 생성
python scripts/verify_trace.py --fix        # trace_map 자동 복구
```

### registry.py

```bash
python scripts/registry.py show                              # 레지스트리 조회
python scripts/registry.py summary PRD-001                   # PRD 추적 요약
python scripts/registry.py prd-complete PRD-001              # PRD 완료 확인
```

## 라이선스

MIT
