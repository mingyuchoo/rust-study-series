# SPEC-011: select/option 다크 테마 CSS

## 메타데이터
- SPEC ID: SPEC-011
- PRD: PRD-011
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향
| PRD | FR | 요구사항 |
|-----|----|---------|
| PRD-011 | FR-1 | color-scheme 다크 |
| PRD-011 | FR-2 | option 색상 |
| PRD-011 | FR-3 | select 커스텀 appearance |

### 역방향
| TC | 시나리오 | FR | 테스트 파일 |
|----|---------|----|-----------|
| TC-1 | index/help 양쪽에 `color-scheme: dark` | FR-1 | crates/eval-harness/src/web/handlers.rs |
| TC-2 | `select option { ... background ... }` 규칙 | FR-2 | crates/eval-harness/src/web/handlers.rs |
| TC-3 | `appearance: none` + `background-image` 화살표 | FR-3 | crates/eval-harness/src/web/handlers.rs |

### 구현 추적
| 파일 | 변경 |
|------|------|
| src/web/index.html | `:root { color-scheme: dark }` + `select { appearance: none; background-image: url("data:image/svg+xml,...") ... }` + `select option { ... }` |
| src/web/help.html | `:root { color-scheme: dark }` (help 페이지에도 일관성) |
| src/web/handlers.rs | SPEC-011 smoke tests 3개 |

## 기술 설계

### CSS 블록 (index.html)
```css
:root { color-scheme: dark; }

select {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  background-color: #1e1e1e;
  color: #e0e0e0;
  border: 1px solid #444;
  border-radius: 4px;
  padding: 6px 28px 6px 10px;           /* 우측에 화살표 공간 확보 */
  font-size: 13px;
  font-family: inherit;
  cursor: pointer;
  /* 인라인 SVG 화살표 (14×8), #f0c419 색 */
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='14' height='8' viewBox='0 0 14 8'><path d='M1 1l6 6 6-6' stroke='%23f0c419' stroke-width='2' fill='none' stroke-linecap='round' stroke-linejoin='round'/></svg>");
  background-repeat: no-repeat;
  background-position: right 10px center;
}
select:hover { border-color: #f0c419; }
select:focus { outline: 2px solid #f0c419; outline-offset: 1px; }

/* 팝업 리스트 항목 */
select option {
  background: #1e1e1e;
  color: #e0e0e0;
  padding: 8px 10px;
}
select option:checked { background: #f0c419; color: #111; }
```

기존 공용 `select, input, textarea, button { ... }` 규칙은 그대로 유지되지만 `select` 전용 규칙이 뒤에 위치하여 오버라이드한다.

### help.html
help 페이지에는 `<select>` 가 없지만 일관성을 위해 `:root { color-scheme: dark; }` 만 추가 (스크롤바도 다크로 렌더).

## 완료 정의
- TC-1~TC-3 통과
- Tauri 실행 시 select 팝업이 다크 배경 + 밝은 텍스트로 렌더 (수동 확인)
- 기존 폼 정렬·높이 변화 없음 (수동 확인)
