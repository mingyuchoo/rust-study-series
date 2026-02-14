//! Helix 스타일 키바인딩을 위한 메시지 타입
//!
//! 이 모듈은 Selection-Action 모델을 구현하기 위한 52개의 메시지 variants를 정의합니다.
//! Vim의 Action-Motion 모델과 달리, Helix는 Selection을 먼저 하고 Action을 수행합니다.

use chrono::NaiveDate;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Quit,
    DismissError,

    // 에디터 - 네비게이션
    EditorMoveLeft,
    EditorMoveRight,
    EditorMoveUp,
    EditorMoveDown,
    EditorWordNext,
    EditorWordPrev,
    EditorWordEnd,

    // 에디터 - Goto 모드
    EditorEnterGotoMode,
    EditorGotoDocStart,
    EditorGotoDocEnd,
    EditorGotoLineStart,
    EditorGotoLineEnd,
    EditorExitSubMode,

    // 에디터 - Insert 모드
    EditorEnterInsert(InsertPosition),
    EditorEnterNormalMode,
    EditorInsertChar(char),
    EditorBackspace,
    EditorNewLine,

    // 에디터 - Selection
    EditorToggleSelection,
    EditorSelectLine,

    // 에디터 - 편집
    EditorDelete,
    EditorChange,
    EditorYank,
    EditorPasteAfter,
    EditorPasteBefore,

    // 에디터 - Undo/Redo
    EditorUndo,
    EditorRedo,

    // 에디터 - 검색
    EditorEnterSearchMode,
    EditorSearchChar(char),
    EditorSearchBackspace,
    EditorSearchNext,
    EditorSearchPrev,
    EditorExecuteSearch,

    // 에디터 - Space 명령
    EditorEnterSpaceMode,
    EditorSpaceSave,
    EditorSpaceQuit,
    EditorSpaceSaveQuit,
    EditorBack,

    // 달력
    CalendarMoveLeft,
    CalendarMoveRight,
    CalendarMoveUp,
    CalendarMoveDown,
    CalendarSelectDate,
    CalendarEnterSpaceMode,
    CalendarExitSubMode,
    CalendarSpaceQuit,
    CalendarSpaceNextMonth,
    CalendarSpacePrevMonth,
    CalendarSpaceNextYear,
    CalendarSpacePrevYear,

    // 파일 I/O 결과
    LoadDiarySuccess(NaiveDate, String),
    LoadDiaryFailed(String),
    SaveDiarySuccess,
    SaveDiaryFailed(String),
    DeleteDiarySuccess(NaiveDate),
    RefreshIndex(HashSet<NaiveDate>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    BeforeCursor,
    AfterCursor,
    LineBelow,
    LineAbove,
}
