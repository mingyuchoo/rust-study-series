# SPEC-010: Google Fonts 링크 + 폰트 스택 교체

## 메타데이터
- SPEC ID: SPEC-010
- PRD: PRD-010
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향
| PRD | FR | 요구사항 |
|-----|----|---------|
| PRD-010 | FR-1 | body sans 스택 |
| PRD-010 | FR-2 | mono 스택 |
| PRD-010 | FR-3 | 두 페이지 link 삽입 |

### 역방향
| TC | 시나리오 | FR | 테스트 파일 |
|----|---------|----|-----------|
| TC-1 | body font-family 에 IBM Plex Sans KR 포함 | FR-1 | crates/eval-harness/src/web/handlers.rs |
| TC-2 | pre font-family 에 IBM Plex Mono 포함 | FR-2 | crates/eval-harness/src/web/handlers.rs |
| TC-3 | index + help 양쪽에 Google Fonts 링크 | FR-3 | crates/eval-harness/src/web/handlers.rs |

### 구현 추적
| 파일 | 변경 |
|------|------|
| src/web/index.html | `<link>` 추가 + CSS body/pre font-family 교체 |
| src/web/help.html | `<link>` 추가 + CSS body/code/pre font-family 교체 |
| src/web/handlers.rs | SPEC-010 smoke tests 3개 |

## 기술 설계

### 폰트 스택
```css
body {
  font-family: 'IBM Plex Sans KR', 'IBM Plex Sans', -apple-system, 'Segoe UI', sans-serif;
}
code, pre, textarea {
  font-family: 'IBM Plex Mono', 'SF Mono', Menlo, monospace;
}
```

### `<link>` 블록 (양쪽 페이지 동일)
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;600&family=IBM+Plex+Sans+KR:wght@400;600&family=IBM+Plex+Sans:wght@400;600&display=swap" rel="stylesheet">
```

## 완료 정의
- TC-1~TC-3 통과
- `cargo run -- serve` 후 브라우저에서 IBM Plex 폰트로 렌더
- 데스크톱 앱도 동일 (WebView 가 CDN 로드)
