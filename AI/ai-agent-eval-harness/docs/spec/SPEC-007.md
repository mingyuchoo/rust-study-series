# SPEC-007: 임베드 help.html + /help 라우트

## 메타데이터
- SPEC ID: SPEC-007
- PRD: PRD-007
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-007 | FR-1 | /help 라우트 |
| PRD-007 | FR-2 | SPA 헤더 버튼 |

### 역방향 추적
| TC ID | 시나리오 | FR | 테스트 파일 |
|-------|---------|----|-----------|
| TC-1 | help 본문 비어있지 않음 + HTML 형식 | FR-1 | crates/eval-harness/src/web/handlers.rs |
| TC-2 | help 본문에 탭/엔드포인트 키워드 포함 | FR-1 | crates/eval-harness/src/web/handlers.rs |
| TC-3 | index.html 에 /help 링크 존재 | FR-2 | crates/eval-harness/src/web/handlers.rs |

### 구현 추적
| 패키지 | 파일 | 심볼 | 관련 FR |
|--------|------|------|---------|
| eval-harness | src/web/help.html | (정적 HTML) | FR-1 |
| eval-harness | src/web/handlers.rs | help, help_html_body | FR-1 |
| eval-harness | src/web/mod.rs | build_router (/help 라우트) | FR-1 |
| eval-harness | src/web/index.html | 헤더 사용안내 링크 | FR-2 |

## 기술 설계

### 신규 함수
```rust
// handlers.rs
const HELP_HTML: &str = include_str!("help.html");
pub async fn help() -> Html<&'static str> { Html(HELP_HTML) }
#[cfg(test)]
pub fn help_html_body() -> &'static str { HELP_HTML }
```

### 라우터
```rust
.route("/help", get(handlers::help))
```

### 헤더 링크 (index.html)
```html
<header>
  <h1>AI Agent Eval Harness</h1>
  ...
  <a href="/help" target="_blank" class="help-btn">사용안내</a>
</header>
```

### help.html 구성
1. **개요** — 프로젝트 한 문단 + 빠른 시작 (CLI `run` / Web `serve`)
2. **탭별 사용법** — 7개 탭 각각에 3~5줄 설명 + 어떤 API 를 호출하는지
3. **REST API 레퍼런스** — 전체 엔드포인트 표 + 예시 curl
4. **CLI ↔ Web 매핑** — 표
5. **주의사항** — 인증 없음, 경로 순회 차단, PPA 환경변수

## 완료 정의
- TC-1~TC-3 통과
- `curl http://localhost:8080/help` → HTML 응답
- SPA 헤더 버튼 클릭 시 새 탭 열림
- README 에 `/help` 한 줄 추가
