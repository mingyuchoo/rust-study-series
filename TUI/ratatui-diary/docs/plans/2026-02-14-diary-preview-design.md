# Diary 미리보기 기능 설계

**작성일:** 2026-02-14
**상태:** 승인됨

## 개요

ratatui-diary 애플리케이션에 실시간 미리보기 기능을 추가합니다. 달력 화면과 에디터 화면 모두에서 화면을 50:50으로 분할하여 오른쪽에 미리보기를 표시합니다.

## 요구사항

### 달력 화면
- 왼쪽 50%: 기존 달력
- 오른쪽 50%: 선택된 날짜의 다이어리 전체 내용 미리보기
- 화살표 키로 날짜 이동 시 실시간 업데이트
- 다이어리가 없으면 안내 메시지 표시

### 에디터 화면
- 왼쪽 50%: 텍스트 에디터
- 오른쪽 50%: Markdown 렌더링 결과
- 타이핑할 때마다 실시간 업데이트
- 고급 Markdown 렌더링: 헤더, 강조, 리스트, 코드 블록, 인용, 수평선, 표, 링크 등

## 아키텍처

### 전체 구조

현재 ELM 패턴(Model-Update-View)을 유지하면서 미리보기 기능을 추가합니다.

### 핵심 변경사항

1. **Model 확장**
   - `Model`에 `show_preview: bool` 필드 추가 (기본값: true)
   - 향후 토글 기능 대비

2. **View 레이아웃 수정**
   - `render_calendar()`: 수평 분할 (50:50)
     - 왼쪽: 기존 달력
     - 오른쪽: 선택된 날짜의 다이어리 텍스트

   - `render_editor()`: 수평 분할 (50:50)
     - 왼쪽: 기존 에디터
     - 오른쪽: Markdown 렌더링 결과

3. **새로운 모듈**
   - `src/markdown.rs`: termimad를 래핑하는 렌더링 유틸리티
     - `render_to_text(content: &str) -> Text`: Markdown을 ratatui 위젯으로 변환
     - `create_markdown_theme() -> TermimadTheme`: 테마 설정

4. **의존성 추가**
   - `Cargo.toml`에 `termimad` 추가

### 레이아웃 다이어그램

```
┌─────────────────────────────────────────────┐
│ Calendar Screen (50:50)                     │
├──────────────────┬──────────────────────────┤
│   Calendar       │   Selected Date Preview  │
│   Grid           │   (Read-only Text)       │
└──────────────────┴──────────────────────────┘

┌─────────────────────────────────────────────┐
│ Editor Screen (50:50)                       │
├──────────────────┬──────────────────────────┤
│   Text Editor    │   Markdown Preview       │
│   (Editable)     │   (Rendered)             │
└──────────────────┴──────────────────────────┘
```

## 컴포넌트

### 2.1 새로운 렌더링 함수들 (`src/view.rs`)

**`fn render_preview_pane(f: &mut Frame, area: Rect, content: &str, title: &str)`**
- 미리보기 영역을 렌더링하는 공통 함수
- 테두리, 제목, 스크롤 가능한 텍스트 표시
- 달력/에디터 화면 모두에서 재사용

**`fn render_markdown_preview(f: &mut Frame, area: Rect, content: &str)`**
- termimad를 사용한 Markdown 렌더링
- `src/markdown.rs`의 유틸리티 호출
- 에디터 화면 전용

### 2.2 Markdown 모듈 (`src/markdown.rs` - 신규)

**`pub fn render_to_text(markdown: &str) -> Text<'static>`**
- Markdown 문자열을 ratatui의 `Text` 위젯으로 변환
- termimad의 파서 + 스타일 매핑
- 지원 요소:
  - 헤더 (# ~ ######): 크기/색상 차등
  - 강조 (**bold**, *italic*, ~~strikethrough~~): 스타일 적용
  - 리스트 (-, *, 1.): 들여쓰기 + 불릿
  - 코드 블록 (```): 회색 배경 + 고정폭 폰트 스타일
  - 인용 (>): 파란색 + 세로 바
  - 수평선 (---): 긴 선 문자
  - 표: 박스 그리기 문자로 테두리
  - 링크: 밑줄 + 청록색

**`pub fn create_markdown_theme() -> TermimadTheme`**
- termimad 테마 설정
- 다이어리 앱에 어울리는 부드러운 색상 팔레트

### 2.3 Storage 확장 (`src/storage.rs`)

기존 `load()` 메서드 활용:
- 달력 화면에서 선택된 날짜의 다이어리를 읽어옴
- 없으면 빈 문자열 또는 기본 메시지 반환 (에러 무시)

### 2.4 Update 로직 (`src/update.rs`)

변경 불필요:
- 미리보기는 읽기 전용이므로 기존 상태 업데이트 로직 그대로 유지
- 에디터에서 문자 입력 시 view가 자동으로 실시간 업데이트됨

## 데이터 플로우

### 3.1 달력 화면 데이터 플로우

```
사용자 입력 (h/j/k/l)
  → update::handle_calendar_key()
  → model.calendar_state.selected_date 변경
  → view::render_calendar()
  ├─ 왼쪽: render_calendar_grid() (기존)
  └─ 오른쪽: render_preview_pane()
       → storage.load(selected_date)
       → 다이어리 내용 또는 "작성된 다이어리가 없습니다" 표시
```

**특징:**
- 선택된 날짜가 바뀔 때마다 storage에서 다이어리를 즉시 로드
- 로드 실패 시 빈 화면이 아닌 안내 메시지 표시
- 성능: 파일 읽기는 충분히 빠름 (수 KB 텍스트 파일)

### 3.2 에디터 화면 데이터 플로우

```
사용자 입력 (문자 입력, Insert 모드)
  → update::handle_editor_key()
  → model.editor_state.content 변경
  → model.editor_state.is_modified = true
  → view::render_editor()
  ├─ 왼쪽: render_editor_content() (기존)
  └─ 오른쪽: render_markdown_preview()
       → markdown::render_to_text(editor_state.get_content())
       → termimad 렌더링
       → 실시간 미리보기 업데이트
```

**특징:**
- 타이핑할 때마다 `get_content()` 호출하여 현재 버퍼 내용을 가져옴
- Markdown 파싱 및 렌더링은 매 프레임 실행 (60 FPS 기준 ~16ms)
- 성능 고려사항:
  - termimad는 충분히 빠름 (수백 줄 문서도 문제없음)
  - 만약 매우 긴 문서(>1000줄)에서 느려지면 debounce 추가 고려

## 에러 처리

### 4.1 Storage 로드 실패 처리

달력 화면에서 다이어리 로드 실패 시:
```rust
let preview_content = match storage.load(selected_date) {
    Ok(content) => content,
    Err(_) => "📝 작성된 다이어리가 없습니다.\n\nEnter를 눌러 새로 작성하세요.".to_string(),
};
```

**특징:**
- 에러 팝업 표시하지 않음 (다이어리가 없는 것은 정상 상태)
- 사용자 친화적인 안내 메시지
- 기존 `show_error_popup` 메커니즘 사용 안 함

### 4.2 Markdown 렌더링 실패 처리

termimad 파싱/렌더링 실패 시:
```rust
pub fn render_to_text(markdown: &str) -> Text<'static> {
    match termimad::parse_text(markdown) {
        Ok(rendered) => rendered,
        Err(e) => {
            // Fallback: 원본 텍스트 그대로 표시
            Text::raw(format!("⚠️  Markdown 렌더링 실패\n\n{}", markdown))
        }
    }
}
```

**특징:**
- 렌더링 실패해도 원본 텍스트는 보여줌
- 앱 크래시 방지
- 에러 메시지 최소화 (사용자 경험 우선)

### 4.3 성능 저하 처리

매우 긴 문서(>1000줄)에서 렌더링 지연 시:
```rust
// 향후 필요시 추가
const MAX_PREVIEW_LINES: usize = 500;

fn truncate_for_preview(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() > MAX_PREVIEW_LINES {
        let truncated = lines[..MAX_PREVIEW_LINES].join("\n");
        format!("{}\n\n... (나머지 {} 줄 생략)", truncated, lines.len() - MAX_PREVIEW_LINES)
    } else {
        content.to_string()
    }
}
```

**특징:**
- 초기 구현에서는 생략 (대부분의 다이어리는 짧음)
- 성능 문제 발생 시에만 추가
- YAGNI 원칙 적용

### 4.4 화면 크기 처리

터미널이 너무 작을 때 (< 80 컬럼):
```rust
// view.rs의 render 함수에서
if f.size().width < 80 {
    // 미리보기 숨기고 전체 화면 사용
    render_calendar_fullscreen(f, model);
} else {
    // 50:50 분할
    render_calendar_with_preview(f, model);
}
```

**특징:**
- 작은 터미널에서도 사용 가능
- 자동 레이아웃 조정
- 사용성 우선

## 테스팅

### 5.1 단위 테스트 (`tests/markdown_tests.rs` - 신규)

- `test_render_headers()`: # H1, ## H2 등이 올바른 스타일로 변환되는지 테스트
- `test_render_emphasis()`: **bold**, *italic*, ~~strike~~ 스타일 테스트
- `test_render_lists()`: - item, * item, 1. item 렌더링 테스트
- `test_render_code_blocks()`: ``` 코드 블록 스타일 테스트
- `test_render_tables()`: | col1 | col2 | 표 렌더링 테스트
- `test_render_empty_string()`: 빈 문자열 처리
- `test_render_invalid_markdown()`: 잘못된 Markdown도 크래시 없이 처리

### 5.2 통합 테스트 (`tests/view_tests.rs` - 신규)

- `test_calendar_preview_with_existing_diary()`: 다이어리가 있는 날짜 선택 시 내용 표시
- `test_calendar_preview_without_diary()`: 다이어리가 없는 날짜 선택 시 안내 메시지
- `test_editor_realtime_preview()`: 에디터에서 타이핑 시 미리보기 업데이트
- `test_small_terminal_fallback()`: 작은 터미널에서 레이아웃 조정

### 5.3 수동 테스트 체크리스트

구현 완료 후 수동으로 확인할 항목:
- [ ] 달력에서 화살표 키로 날짜 이동 시 미리보기 즉시 업데이트
- [ ] 다이어리가 있는 날짜/없는 날짜 모두 정상 표시
- [ ] 에디터에서 Markdown 입력 시 실시간 렌더링 (헤더, 리스트, 코드 블록 등)
- [ ] 긴 다이어리(100+ 줄) 스크롤 가능
- [ ] 작은 터미널(< 80 컬럼)에서도 정상 동작
- [ ] 한글, 이모지 등 유니코드 문자 정상 렌더링
- [ ] 성능: 타이핑 시 지연 없음 (< 50ms)

### 5.4 테스트 커버리지 목표

- Markdown 렌더링 함수: **80%+**
- View 렌더링 로직: **60%+** (ratatui 통합으로 일부 테스트 어려움)
- 전체 프로젝트: **70%+**

## 구현 우선순위

1. **Phase 1: 기본 구조**
   - termimad 의존성 추가
   - `src/markdown.rs` 모듈 생성 및 기본 렌더링 함수
   - View 레이아웃 50:50 분할

2. **Phase 2: 달력 미리보기**
   - 달력 화면에 텍스트 미리보기 추가
   - Storage 통합
   - 에러 처리 (다이어리 없을 때)

3. **Phase 3: 에디터 Markdown 미리보기**
   - 에디터 화면에 Markdown 렌더링 추가
   - 실시간 업데이트 구현
   - 고급 Markdown 요소 지원

4. **Phase 4: 폴리싱**
   - 작은 터미널 대응
   - 테스트 작성
   - 성능 최적화 (필요시)

## 향후 개선 가능성

- 미리보기 토글 키바인딩 (예: `p` 키)
- 미리보기 비율 조정 가능 (40:60, 30:70 등)
- 다크/라이트 테마 지원
- 커스텀 Markdown 스타일 설정

## 기술 스택

- **Markdown 파싱/렌더링:** termimad
- **UI 프레임워크:** ratatui (기존)
- **아키텍처:** ELM 패턴 (Model-Update-View)

## 승인

이 설계는 2026-02-14에 사용자에게 승인되었습니다.
