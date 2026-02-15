# 키바인딩 도움말 시스템

## 개요

화면 하단에 현재 모드/서브모드에 맞는 키바인딩 도움말을 동적으로 표시합니다.

## 기능

### 달력 화면

- **Normal 모드**: `hjkl:이동 | e:편집 | space:명령 | q:종료`
- **Space 모드**: `n/p:다음/이전달 | y/Y:다음/이전년 | q:종료 | Esc:취소`

### 에디터 화면

- **Normal 모드**: 모든 주요 키바인딩 표시
- **Insert 모드**: Insert 모드 관련 키만 표시
- **Goto 모드**: Goto 명령 키 표시
- **Space 모드**: Space 명령 키 표시
- **Search 모드**: 검색 관련 키 표시

## 구현

- 파일: `src/view.rs`
- 함수: `build_calendar_keybindings()`, `build_editor_keybindings()`
- 테스트: `tests/view_tests.rs`

## 구현 세부사항

### 달력 화면 키바인딩

`build_calendar_keybindings()` 함수는 현재 서브모드에 따라 다른 키바인딩을 표시합니다:

- **Normal 모드**: 기본 네비게이션 및 편집 키
- **Space 모드**: 월/년도 이동 및 종료 명령

### 에디터 화면 키바인딩

`build_editor_keybindings()` 함수는 에디터 모드에 따라 다양한 키바인딩을 표시합니다:

- **Normal 모드**: 이동, Selection, 편집, Undo/Redo, 검색 등 모든 기능
- **Insert 모드**: 텍스트 입력 관련 키만 표시
- **Goto 모드**: `gg`, `ge`, `gh`, `gl` 등 Goto 명령
- **Space 모드**: 저장, 종료 명령
- **Search 모드**: 검색 진행 중 표시

## 테스트 커버리지

모든 모드 및 서브모드에 대한 유닛 테스트가 포함되어 있습니다:

- 달력 Normal 모드
- 달력 Space 모드
- 에디터 모든 모드 (Normal, Insert, Goto, Space, Search)

## 사용자 경험

키바인딩 도움말은 화면 하단에 항상 표시되어 사용자가 현재 컨텍스트에서 사용 가능한 키를 쉽게 확인할 수 있습니다. 이는 특히 Helix 스타일의 모달 에디터를 처음 사용하는 사용자에게 유용합니다.
