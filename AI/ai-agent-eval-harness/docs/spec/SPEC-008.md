# SPEC-008: data-i18n 사전 + lang-ko/lang-en 블록

## 메타데이터
- SPEC ID: SPEC-008
- PRD: PRD-008
- 작성일: 2026-04-10
- 상태: Draft

## 추적 정보

### 정방향
| PRD | FR | 요구사항 |
|-----|----|---------|
| PRD-008 | FR-1 | ko/en 토글 버튼 + 영속화 |
| PRD-008 | FR-2 | index.html i18n 사전 전환 |
| PRD-008 | FR-3 | help.html 듀얼 블록 전환 |

### 역방향
| TC | 시나리오 | FR | 테스트 파일 |
|----|---------|----|-----------|
| TC-1 | 토글 버튼 존재 (`id="lang-ko"` + `id="lang-en"`) | FR-1 | handlers.rs |
| TC-2 | `setLang(` 함수 + `localStorage` 사용 | FR-1 | handlers.rs |
| TC-3 | `const I18N =` 사전 + `ko:` + `en:` 키 | FR-2 | handlers.rs |
| TC-4 | `data-i18n="` 속성 10개 이상 | FR-2 | handlers.rs |
| TC-5 | help.html 에 `lang-ko` + `lang-en` 블록 둘 다 | FR-3 | handlers.rs |

### 구현 추적
| 파일 | 변경 |
|------|------|
| src/web/index.html | 헤더 토글 추가, data-i18n 마킹, I18N 사전, setLang() |
| src/web/help.html | 헤더 토글 추가, main 블록 2개 (lang-ko/lang-en), setLang() |
| src/web/handlers.rs | SPEC-008 smoke tests 5개 |

## 기술 설계

### 공용 JS 패턴
```js
function setLang(lang) {
  localStorage.setItem('lang', lang);
  document.documentElement.lang = lang;
  // update toggle button active state
  document.querySelectorAll('.lang-btn').forEach(b => b.classList.toggle('active', b.dataset.lang === lang));
  // index.html: apply via dictionary
  // help.html: toggle visibility of .lang-ko / .lang-en blocks
}
```

### index.html 사전 예 (키 네이밍: `section.subject`)
```js
const I18N = {
  ko: {
    "header.sub": "— REST API 싱글 페이지 클라이언트",
    "header.help": "📖 사용안내",
    "nav.run": "실행", "nav.scenarios": "시나리오", "nav.tools": "도구",
    "nav.agents": "에이전트", "nav.reports": "리포트",
    "nav.trajectories": "궤적", "nav.goldens": "골든셋",
    "run.title": "벤치마크 스위트 실행",
    ...
  },
  en: {
    "header.sub": "— single-page client for the REST API",
    "header.help": "📖 Help",
    "nav.run": "Run", "nav.scenarios": "Scenarios", ...
  }
};
```

### help.html 듀얼 블록
```html
<main class="lang-ko">... 한국어 전체 본문 ...</main>
<main class="lang-en">... English full text ...</main>
```
CSS: `main.lang-en { display: none; }` 가 기본, `html[lang="en"] main.lang-ko { display: none; }` + `html[lang="en"] main.lang-en { display: block; }`.

## 완료 정의
- TC-1~TC-5 통과
- 실행 후 수동 확인: 토글 클릭 시 모든 문자열 즉시 전환 + 새로고침 후 유지
