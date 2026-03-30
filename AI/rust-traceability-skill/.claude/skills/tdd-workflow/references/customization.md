# 커스터마이징 가이드

## 프로젝트별 설정

### 디렉토리 구조 변경

기본 구조가 프로젝트와 맞지 않으면 SKILL.md 의 "디렉토리 규약" 섹션을 수정한다.

예시 - 모노레포 구조:
```
monorepo/
├── packages/
│   ├── api/
│   │   ├── docs/prd/
│   │   ├── docs/spec/
│   │   ├── src/
│   │   └── tests/
│   └── web/
│       ├── docs/prd/
│       ├── docs/spec/
│       ├── src/
│       └── tests/
└── docs/registry.json    # 전체 공유
```

### ID 형식 변경

기본: `PRD-001`, `SPEC-001` (3자리)

프로젝트 접두사를 추가하려면:
- `{PROJECT}-PRD-001` (예: `AUTH-PRD-001`)
- registry.py 의 next_prd_id, next_spec_id 함수를 수정한다.

### 테스트 프레임워크 지정

Phase 0 에서 자동 감지하지만, 명시적으로 지정하고 싶으면 
SKILL.md 의 Phase 0 에 다음을 추가한다:

```
프로젝트 설정:
- 언어: TypeScript
- 테스트 러너: vitest
- 테스트 명령: npx vitest run
- 테스트 디렉토리: src/__tests__/
```

### PRD/SPEC 템플릿 수정

templates/ 디렉토리에 커스텀 템플릿을 넣으면 SKILL.md 의 기본 템플릿 대신 사용할 수 있다.
SKILL.md 의 Phase 1, Phase 2 에서 "템플릿" 부분을 수정하여 
`templates/prd-template.md`, `templates/spec-template.md` 를 읽도록 변경한다.

## 워크플로우 확장

### CI 연동

Phase 5 이후에 CI 파이프라인 트리거를 추가할 수 있다:

```bash
# GitHub Actions 트리거
git add docs/ tests/ src/
git commit -m "feat(SPEC-{ID}): {기능명}

PRD: PRD-{PRD_ID}
SPEC: SPEC-{SPEC_ID}"
git push
```

### 코드 리뷰 연동

gstack 의 `/review` 와 결합하려면 Phase 5 이후에:
1. 브랜치를 만들고 커밋한다
2. `/review` 를 실행한다
3. 리뷰 결과를 반영한다

### 상태 추적

registry.json 의 상태값:
- `draft` - 초안 작성됨
- `review` - 리뷰 중
- `approved` - 승인됨
- `implementing` - 구현 중
- `implemented` - 구현 완료
- `tested` - 테스트 통과
- `released` - 배포 완료
