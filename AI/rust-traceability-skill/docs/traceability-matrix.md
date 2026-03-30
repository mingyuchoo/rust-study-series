# 추적성 매트릭스

최종 갱신: 2026-03-31

## 정방향 추적 (요구사항 -> 구현)

| PRD | FR ID | FR 제목 | SPEC | TC | 테스트 파일 | 구현 파일 | 구현 심볼 | 테스트 상태 |
|-----|-------|--------|------|-----|-----------|----------|----------|-----------|
| PRD-001 | FR-1 | 게시글 생성 | SPEC-001 | TC-1, TC-2 | tests/test_blog_api.rs | src/store.rs, src/handler.rs | create_post | PASS |
| PRD-001 | FR-2 | 게시글 단건 조회 | SPEC-001 | TC-3, TC-4 | tests/test_blog_api.rs | src/store.rs, src/handler.rs | get_post | PASS |
| PRD-001 | FR-3 | 게시글 목록 조회 | SPEC-001 | TC-5, TC-6 | tests/test_blog_api.rs | src/store.rs, src/handler.rs | list_posts | PASS |
| PRD-001 | FR-4 | 게시글 수정 | SPEC-001 | TC-7, TC-8 | tests/test_blog_api.rs | src/store.rs, src/handler.rs | update_post | PASS |
| PRD-001 | FR-5 | 게시글 삭제 | SPEC-001 | TC-9, TC-10 | tests/test_blog_api.rs | src/store.rs, src/handler.rs | delete_post | PASS |

## 역방향 추적 (구현 -> 요구사항)

| 구현 파일 | 심볼 | SPEC | TC | FR | PRD | 테스트 상태 |
|----------|------|------|-----|-----|-----|-----------|
| src/store.rs | create_post | SPEC-001 | TC-1, TC-2 | PRD-001/FR-1 | PRD-001 | PASS |
| src/store.rs | get_post | SPEC-001 | TC-3, TC-4 | PRD-001/FR-2 | PRD-001 | PASS |
| src/store.rs | list_posts | SPEC-001 | TC-5, TC-6 | PRD-001/FR-3 | PRD-001 | PASS |
| src/store.rs | update_post | SPEC-001 | TC-7, TC-8 | PRD-001/FR-4 | PRD-001 | PASS |
| src/store.rs | delete_post | SPEC-001 | TC-9, TC-10 | PRD-001/FR-5 | PRD-001 | PASS |
| src/handler.rs | create_post | SPEC-001 | TC-1, TC-2 | PRD-001/FR-1 | PRD-001 | PASS |
| src/handler.rs | get_post | SPEC-001 | TC-3, TC-4 | PRD-001/FR-2 | PRD-001 | PASS |
| src/handler.rs | list_posts | SPEC-001 | TC-5, TC-6 | PRD-001/FR-3 | PRD-001 | PASS |
| src/handler.rs | update_post | SPEC-001 | TC-7, TC-8 | PRD-001/FR-4 | PRD-001 | PASS |
| src/handler.rs | delete_post | SPEC-001 | TC-9, TC-10 | PRD-001/FR-5 | PRD-001 | PASS |

## 커버리지 요약

| PRD | 전체 FR | 커버된 FR | SPEC 수 | TC 수 | 통과 | 실패 | 커버리지 |
|-----|--------|----------|--------|-------|------|------|---------|
| PRD-001 | 5 | 5 | 1 | 10 | 10 | 0 | 100% |

## 미추적 항목 (경고)

없음
