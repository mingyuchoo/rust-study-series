# 추적성 매트릭스

최종 갱신: 2026-03-31

## 정방향 추적 (요구사항 -> 구현)

| PRD | FR ID | FR 제목 | SPEC | TC | 테스트 파일 | 구현 파일 | 구현 심볼 | 테스트 상태 |
|-----|-------|--------|------|-----|-----------|----------|----------|-----------|
| PRD-001 | FR-1 | 여러 타임존의 현재 시간 표시 | SPEC-001 | TC-1, TC-2, TC-3, TC-4 | tests/test_clock.rs | src/clock.rs, src/config.rs | get_clock_display, format_clocks, Config::load | PASS |
| PRD-001 | FR-2 | 도시/타임존 추가 | SPEC-001 | TC-5, TC-6, TC-7, TC-10 | tests/test_config.rs | src/config.rs | Config::add, Config::save | PASS |
| PRD-001 | FR-3 | 도시/타임존 삭제 | SPEC-001 | TC-8, TC-9, TC-10 | tests/test_config.rs | src/config.rs | Config::remove, Config::save | PASS |
| PRD-001 | FR-4 | 저장된 도시 목록 조회 | SPEC-001 | TC-11, TC-12 | tests/test_config.rs | src/config.rs | Config::load | PASS |
| PRD-002 | FR-1 | REST API로 모든 도시의 현재 시간 조회 | SPEC-002 | TC-1, TC-2 | tests/test_web.rs | src/web.rs | create_router, get_clocks, AppState | PASS |
| PRD-002 | FR-2 | REST API로 도시 추가 | SPEC-002 | TC-3, TC-4, TC-5 | tests/test_web.rs | src/web.rs | add_city | PASS |
| PRD-002 | FR-3 | REST API로 도시 삭제 | SPEC-002 | TC-6, TC-7 | tests/test_web.rs | src/web.rs | remove_city | PASS |
| PRD-002 | FR-4 | REST API로 도시 목록 조회 | SPEC-002 | TC-8 | tests/test_web.rs | src/web.rs | list_cities | PASS |
| PRD-002 | FR-5 | 웹 서버 시작/종료 | SPEC-002 | TC-9 | tests/test_web.rs | src/cli.rs, src/main.rs | Commands::Serve, run | PASS |
| PRD-003 | FR-1 | 루트 경로(/)에서 HTML 페이지 제공 | SPEC-003 | TC-1, TC-2, TC-3, TC-6 | tests/test_frontend.rs | src/web.rs | index_html, INDEX_HTML, create_router | PASS |
| PRD-003 | FR-2 | 웹 페이지에서 시간 실시간 표시 | SPEC-003 | TC-2, TC-4 | tests/test_frontend.rs | src/web.rs | INDEX_HTML | PASS |
| PRD-003 | FR-3 | 웹 페이지에서 도시 추가 | SPEC-003 | TC-3, TC-4 | tests/test_frontend.rs | src/web.rs | INDEX_HTML | PASS |
| PRD-003 | FR-4 | 웹 페이지에서 도시 삭제 | SPEC-003 | TC-4, TC-5 | tests/test_frontend.rs | src/web.rs | INDEX_HTML | PASS |
| PRD-004 | FR-1 | 추적성 데이터를 JSON API로 제공 | SPEC-004 | TC-1, TC-2 | tests/test_trace.rs | src/web.rs | get_trace, AppState | PASS |
| PRD-004 | FR-2 | 추적성 그래프 웹 페이지 제공 | SPEC-004 | TC-3, TC-4, TC-8 | tests/test_trace.rs | src/web.rs | trace_html, TRACE_HTML, create_router | PASS |
| PRD-004 | FR-3 | 정방향 추적 그래프 표시 | SPEC-004 | TC-4, TC-5, TC-7 | tests/test_trace.rs | src/web.rs | TRACE_HTML | PASS |
| PRD-004 | FR-4 | 역방향 추적 그래프 표시 | SPEC-004 | TC-6, TC-7 | tests/test_trace.rs | src/web.rs | TRACE_HTML | PASS |

## 역방향 추적 (구현 -> 요구사항)

| 구현 파일 | 심볼 | SPEC | TC | FR | PRD | 테스트 상태 |
|----------|------|------|-----|-----|-----|-----------|
| src/clock.rs | get_clock_display | SPEC-001 | TC-1, TC-2 | PRD-001/FR-1 | PRD-001 | PASS |
| src/clock.rs | format_clocks | SPEC-001 | TC-3, TC-4 | PRD-001/FR-1 | PRD-001 | PASS |
| src/config.rs | Config::load | SPEC-001 | TC-10, TC-11 | PRD-001/FR-1, PRD-001/FR-4 | PRD-001 | PASS |
| src/config.rs | Config::save | SPEC-001 | TC-10 | PRD-001/FR-2, PRD-001/FR-3 | PRD-001 | PASS |
| src/config.rs | Config::add | SPEC-001 | TC-5, TC-6, TC-7 | PRD-001/FR-2 | PRD-001 | PASS |
| src/config.rs | Config::remove | SPEC-001 | TC-8, TC-9 | PRD-001/FR-3 | PRD-001 | PASS |
| src/config.rs | default_config_path | SPEC-001 | - | PRD-001/FR-2, PRD-001/FR-3 | PRD-001 | - |
| src/error.rs | AppError | SPEC-001 | - | PRD-001/FR-1~3 | PRD-001 | - |
| src/cli.rs | Cli, Commands | SPEC-001, SPEC-002 | - | PRD-001/FR-1~4, PRD-002/FR-5 | PRD-001, PRD-002 | - |
| src/web.rs | AppState | SPEC-002, SPEC-004 | TC-1~TC-8, TC-1~TC-2 | PRD-002/FR-1~4, PRD-004/FR-1 | PRD-002, PRD-004 | PASS |
| src/web.rs | create_router | SPEC-002, SPEC-003, SPEC-004 | - | PRD-002/FR-1~4, PRD-003/FR-1, PRD-004/FR-1~2 | PRD-002, PRD-003, PRD-004 | PASS |
| src/web.rs | get_clocks | SPEC-002 | TC-1, TC-2 | PRD-002/FR-1 | PRD-002 | PASS |
| src/web.rs | add_city | SPEC-002 | TC-3, TC-4, TC-5 | PRD-002/FR-2 | PRD-002 | PASS |
| src/web.rs | remove_city | SPEC-002 | TC-6, TC-7 | PRD-002/FR-3 | PRD-002 | PASS |
| src/web.rs | list_cities | SPEC-002 | TC-8 | PRD-002/FR-4 | PRD-002 | PASS |
| src/web.rs | index_html | SPEC-003 | TC-1~TC-5 | PRD-003/FR-1~4 | PRD-003 | PASS |
| src/web.rs | INDEX_HTML | SPEC-003 | TC-2~TC-5 | PRD-003/FR-1~4 | PRD-003 | PASS |
| src/web.rs | get_trace | SPEC-004 | TC-1, TC-2 | PRD-004/FR-1 | PRD-004 | PASS |
| src/web.rs | trace_html | SPEC-004 | TC-3~TC-7 | PRD-004/FR-2, PRD-004/FR-3, PRD-004/FR-4 | PRD-004 | PASS |
| src/web.rs | TRACE_HTML | SPEC-004 | TC-4~TC-7 | PRD-004/FR-2~4 | PRD-004 | PASS |

## 커버리지 요약

| PRD | 전체 FR | 커버된 FR | SPEC 수 | TC 수 | 통과 | 실패 | 커버리지 |
|-----|--------|----------|--------|-------|------|------|---------|
| PRD-001 | 4 | 4 | 1 | 12 | 12 | 0 | 100% |
| PRD-002 | 5 | 5 | 1 | 9 | 9 | 0 | 100% |
| PRD-003 | 4 | 4 | 1 | 6 | 6 | 0 | 100% |
| PRD-004 | 4 | 4 | 1 | 8 | 8 | 0 | 100% |

## 미추적 항목 (경고)

없음
