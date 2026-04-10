# SPEC-014: 웹 UI 라이트/다크 테마 토글

## 메타데이터
- SPEC ID: SPEC-014
- PRD: PRD-014
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-014 | FR-1 | 상단 헤더에 라이트/다크 테마 토글 버튼을 제공한다 |
| PRD-014 | FR-2 | 토글 선택에 따라 모든 화면 컴포넌트의 스타일이 해당 테마로 일관되게 변경된다 |
| PRD-014 | FR-3 | 선택된 테마는 브라우저 저장소에 영속되어 재접속 시 유지된다 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1 | index.html 에 theme-switch 컨테이너와 라이트/다크(data-theme) 버튼이 존재 | FR-1 | crates/eval-harness/src/web_theme.rs | Draft |
| TC-2 | `index.html` 이 CSS 변수 기반 테마(`[data-theme="light"]` 선택자와 `--bg`, `--fg` 등 변수)를 정의 | FR-2 | crates/eval-harness/src/web_theme.rs | Draft |
| TC-3 | `index.html` 에 `setTheme` 함수와 `localStorage.setItem('theme', ...)` 호출이 존재 | FR-3 | crates/eval-harness/src/web_theme.rs | Draft |
| TC-4 | `index.html` 초기화 시 `localStorage.getItem('theme')` 로 저장된 테마를 로드 | FR-3 | crates/eval-harness/src/web_theme.rs | Draft |
| TC-5 | `help.html` 에도 동일한 `theme-switch` 와 CSS 변수 기반 테마가 적용 | FR-2 | crates/eval-harness/src/web_theme.rs | Draft |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 패키지 | 파일 | 심볼 (함수/클래스) | 관련 FR |
|--------|------|-------------------|--------|
| crates/eval-harness | crates/eval-harness/src/web/index.html | `#theme-switch`, `setTheme`, `initTheme` | FR-1, FR-2, FR-3 |
| crates/eval-harness | crates/eval-harness/src/web/help.html | `#theme-switch`, `setTheme`, `initTheme` | FR-1, FR-2, FR-3 |
| crates/eval-harness | crates/eval-harness/src/web_theme.rs | `WEB_INDEX_HTML`, `WEB_HELP_HTML` (테스트 대상 상수) | (test) |

## 개요
정적 SPA 페이지 두 개(`index.html`, `help.html`)에 라이트/다크 테마 토글을 추가한다. 기존에는 다크 톤이 하드코딩되어 있었으며, 이를 CSS 변수 기반으로 리팩터링하여 `data-theme="light|dark"` 속성으로 전환 가능하도록 만든다. 선택된 테마는 `localStorage.theme` 에 저장되어 두 페이지가 공유한다.

## 기술 설계

### CSS 변수 팔레트
`:root[data-theme="dark"]` 와 `:root[data-theme="light"]` 두 블록에 동일 변수 세트를 정의:

```
--bg            배경
--bg-elev       헤더/카드/리스트 배경
--bg-input      입력/버튼 배경
--bg-pre        코드 블록 배경
--fg            본문 텍스트
--fg-muted      부가 텍스트
--accent        포인트 색 (#f0c419 유지)
--accent-hover  포인트 호버
--border        테두리
--link          링크 색
--err           에러 텍스트
--ok            성공 텍스트
```

모든 하드코딩된 hex 색을 `var(--*)` 로 치환한다.

### 토글 UI
언어 전환 버튼 옆에 동일 스타일 패턴으로 `#theme-switch` 컨테이너를 추가:

```html
<div class="theme-switch" id="theme-switch">
  <button class="theme-btn" data-theme="light">☀</button>
  <button class="theme-btn" data-theme="dark">☾</button>
</div>
```

### JS 동작
```js
function currentTheme() { return localStorage.getItem('theme') || 'dark'; }
function setTheme(theme) {
  localStorage.setItem('theme', theme);
  document.documentElement.setAttribute('data-theme', theme);
  document.querySelectorAll('.theme-btn').forEach(b =>
    b.classList.toggle('active', b.dataset.theme === theme));
}
function initTheme() { setTheme(currentTheme()); }
```

`initTheme()` 은 페이지 로드 시 즉시 호출된다. `help.html` 도 동일한 로직을 포함하므로 두 페이지 간 상태가 공유된다(동일 origin, 동일 localStorage).

### 검증 전략
테스트는 Rust 단위 테스트로, `include_str!` 로 HTML 콘텐츠를 로드해 필수 토큰(CSS 변수, data-theme 속성, localStorage 호출, 버튼 셀렉터)이 존재하는지 문자열 매칭으로 검증한다. 렌더링 테스트는 범위 외.

## 대상 패키지
- `crates/eval-harness`: 웹 정적 자산(`src/web/*.html`) 및 검증용 테스트 모듈.

## 테스트 시나리오
| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | index.html 에 테마 토글 버튼 존재 | index.html 문자열 | `id="theme-switch"` 및 `data-theme="light"`, `data-theme="dark"` 버튼 포함 | 단위 | FR-1 |
| TC-2 | index.html 이 CSS 변수 테마 정의 | index.html 문자열 | `[data-theme="light"]` 와 `--bg`, `--fg`, `--accent` CSS 변수 포함 | 단위 | FR-2 |
| TC-3 | index.html 에 setTheme + localStorage 저장 로직 | index.html 문자열 | `setTheme` 및 `localStorage.setItem('theme'` 문자열 포함 | 단위 | FR-3 |
| TC-4 | index.html 이 저장된 테마 로드 | index.html 문자열 | `localStorage.getItem('theme')` 호출 포함 | 단위 | FR-3 |
| TC-5 | help.html 에도 동일 테마 시스템 적용 | help.html 문자열 | `id="theme-switch"` 와 `[data-theme="light"]` + setTheme 포함 | 단위 | FR-2 |

## 구현 가이드
- 대상 패키지: `crates/eval-harness`
- 파일 위치:
  - `crates/eval-harness/src/web/index.html` — CSS 변수화, 토글 UI, JS
  - `crates/eval-harness/src/web/help.html` — 동일 적용
  - `crates/eval-harness/src/web_theme.rs` — 신규 테스트 모듈 (`include_str!` 로 HTML 로드 + assert)
  - `crates/eval-harness/src/lib.rs` — `pub mod web_theme;` 추가 (없으면 생성)
- 의존성: 신규 런타임 의존성 없음. 기존 std 만 사용.
- 주의사항:
  - 기존 i18n(`data-i18n`)과 탭 네비게이션 동작을 깨지 않는다.
  - 색상 변수는 의미 있는 이름으로 명명(`--bg`, `--fg`, `--accent` 등)하여 두 테마 모두에서 동일 구조.
  - 라이트 테마에서도 포인트 색 `#f0c419` 계열은 유지하되 명도 대비가 확보되도록 조정.

## 완료 정의 (Definition of Done)
- [ ] 5 개 TC 모두 통과 (`cargo test -p eval-harness web_theme`)
- [ ] `index.html`/`help.html` 브라우저에서 토글 동작 확인
- [ ] 새로고침 후 테마 유지
- [ ] 추적성 검증(`verify_trace.py`) 통과
