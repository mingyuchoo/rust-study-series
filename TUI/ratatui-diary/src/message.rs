use chrono::NaiveDate;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Msg {
    // 앱 제어
    Quit,
    Tick,
    DismissError,

    // 달력 네비게이션
    CalendarMoveUp,
    CalendarMoveDown,
    CalendarMoveLeft,
    CalendarMoveRight,
    CalendarPrevMonth,
    CalendarNextMonth,
    CalendarPrevYear,
    CalendarNextYear,
    CalendarSelectDate,

    // 에디터
    EditorEnterInsertMode,
    EditorEnterNormalMode,
    EditorInsertChar(char),
    EditorBackspace,
    EditorNewLine,
    EditorDeleteLine,
    EditorMoveCursor(Direction),
    EditorStartCommand,
    EditorCommandChar(char),
    EditorExecuteCommand,
    EditorBack,

    // 파일 I/O
    LoadDiarySuccess(NaiveDate, String),
    LoadDiaryFailed(String),
    SaveDiarySuccess,
    SaveDiaryFailed(String),
    DeleteDiarySuccess(NaiveDate),
    RefreshIndex(HashSet<NaiveDate>),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
