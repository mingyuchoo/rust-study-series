use crossterm::{event::{self,
                        Event,
                        KeyCode,
                        KeyEvent},
                execute,
                terminal::{EnterAlternateScreen,
                           LeaveAlternateScreen,
                           disable_raw_mode,
                           enable_raw_mode}};
use ratatui::{Terminal,
              backend::CrosstermBackend,
              prelude::*};
use ratatui_diary::{Model,
                    Msg,
                    storage::Storage,
                    update,
                    view};
use std::{io,
          time::Duration};

fn main() -> std::io::Result<()> {
    // Storage 초기화
    let storage = Storage::new()?;
    let entries = storage.scan_entries()?;

    // Model 초기화
    let mut model = Model::new(entries, storage);

    // Terminal 초기화
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // 이벤트 루프
    let result = run_app(&mut terminal, &mut model);

    // Terminal 복원
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, model: &mut Model) -> std::io::Result<()> {
    loop {
        // 렌더링
        terminal.draw(|f| view::view(f, model))?;

        // 이벤트 처리
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && let Some(msg) = handle_key(key, model)
        {
            // Quit 메시지 처리
            if matches!(msg, Msg::Quit) {
                break;
            }

            // Update 호출
            if let Some(cmd) = update::update(model, msg) {
                execute_command(cmd, model)?;
            }
        }
    }

    Ok(())
}

fn handle_key(key: KeyEvent, model: &Model) -> Option<Msg> {
    use ratatui_diary::model::Screen;

    // 에러 팝업이 표시 중이면 Esc로 닫기
    if model.show_error_popup && key.code == KeyCode::Esc {
        return Some(Msg::DismissError);
    }

    match model.screen {
        | Screen::Calendar => handle_calendar_key(key, &model.calendar_state),
        | Screen::Editor => handle_editor_key(key, &model.editor_state),
    }
}

fn handle_calendar_key(key: KeyEvent, state: &ratatui_diary::model::CalendarState) -> Option<Msg> {
    use ratatui_diary::model::CalendarSubMode;

    // Space 서브모드 처리
    if let Some(CalendarSubMode::Space) = state.submode {
        return match key.code {
            | KeyCode::Char('q') => Some(Msg::Quit),
            | KeyCode::Char('n') => Some(Msg::CalendarSpaceNextMonth),
            | KeyCode::Char('p') => Some(Msg::CalendarSpacePrevMonth),
            | KeyCode::Char('y') => Some(Msg::CalendarSpaceNextYear),
            | KeyCode::Char('Y') => Some(Msg::CalendarSpacePrevYear),
            | KeyCode::Esc => Some(Msg::CalendarExitSubMode),
            | _ => None,
        };
    }

    // Normal 키
    match key.code {
        | KeyCode::Char('h') => Some(Msg::CalendarMoveLeft),
        | KeyCode::Char('l') => Some(Msg::CalendarMoveRight),
        | KeyCode::Char('k') => Some(Msg::CalendarMoveUp),
        | KeyCode::Char('j') => Some(Msg::CalendarMoveDown),
        | KeyCode::Enter => Some(Msg::CalendarSelectDate),
        | KeyCode::Char(' ') => Some(Msg::CalendarEnterSpaceMode),
        | KeyCode::Char('q') => Some(Msg::Quit),
        | _ => None,
    }
}

fn handle_editor_key(key: KeyEvent, state: &ratatui_diary::model::EditorState) -> Option<Msg> {
    use ratatui_diary::model::EditorMode;

    match state.mode {
        | EditorMode::Normal => handle_editor_normal_key(key, state),
        | EditorMode::Insert => handle_editor_insert_key(key),
    }
}

fn handle_editor_normal_key(key: KeyEvent, state: &ratatui_diary::model::EditorState) -> Option<Msg> {
    use ratatui_diary::model::EditorSubMode;

    // 서브모드 처리
    match &state.submode {
        | Some(EditorSubMode::Goto) => {
            return match key.code {
                | KeyCode::Char('g') => Some(Msg::EditorGotoDocStart),
                | KeyCode::Char('e') => Some(Msg::EditorGotoDocEnd),
                | KeyCode::Char('h') => Some(Msg::EditorGotoLineStart),
                | KeyCode::Char('l') => Some(Msg::EditorGotoLineEnd),
                | KeyCode::Esc => Some(Msg::EditorExitSubMode),
                | _ => None,
            };
        },
        | Some(EditorSubMode::SpaceCommand) => {
            return match key.code {
                | KeyCode::Char('w') => Some(Msg::EditorSpaceSave),
                | KeyCode::Char('q') => Some(Msg::EditorSpaceQuit),
                | KeyCode::Char('x') => Some(Msg::EditorSpaceSaveQuit),
                | KeyCode::Esc => Some(Msg::EditorExitSubMode),
                | _ => None,
            };
        },
        | Some(EditorSubMode::Search) => {
            return match key.code {
                | KeyCode::Char(c) => Some(Msg::EditorSearchChar(c)),
                | KeyCode::Enter => Some(Msg::EditorExecuteSearch),
                | KeyCode::Esc => Some(Msg::EditorExitSubMode),
                | KeyCode::Backspace => Some(Msg::EditorSearchBackspace),
                | _ => None,
            };
        },
        | None => {},
    }

    // Normal 모드 키
    match key.code {
        // 이동
        | KeyCode::Char('h') => Some(Msg::EditorMoveLeft),
        | KeyCode::Char('l') => Some(Msg::EditorMoveRight),
        | KeyCode::Char('k') => Some(Msg::EditorMoveUp),
        | KeyCode::Char('j') => Some(Msg::EditorMoveDown),
        | KeyCode::Char('w') => Some(Msg::EditorWordNext),
        | KeyCode::Char('b') => Some(Msg::EditorWordPrev),
        | KeyCode::Char('e') => Some(Msg::EditorWordEnd),

        // 서브모드 진입
        | KeyCode::Char('g') => Some(Msg::EditorEnterGotoMode),
        | KeyCode::Char(' ') => Some(Msg::EditorEnterSpaceMode),
        | KeyCode::Char('/') => Some(Msg::EditorEnterSearchMode),

        // Insert
        | KeyCode::Char('i') => Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor)),
        | KeyCode::Char('a') => Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::AfterCursor)),
        | KeyCode::Char('o') => Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::LineBelow)),
        | KeyCode::Char('O') => Some(Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::LineAbove)),

        // Selection
        | KeyCode::Char('v') => Some(Msg::EditorToggleSelection),
        | KeyCode::Char('x') => Some(Msg::EditorSelectLine),

        // 편집
        | KeyCode::Char('d') => Some(Msg::EditorDelete),
        | KeyCode::Char('c') => Some(Msg::EditorChange),
        | KeyCode::Char('y') => Some(Msg::EditorYank),
        | KeyCode::Char('p') => Some(Msg::EditorPasteAfter),
        | KeyCode::Char('P') => Some(Msg::EditorPasteBefore),

        // Undo/Redo
        | KeyCode::Char('u') => Some(Msg::EditorUndo),
        | KeyCode::Char('U') => Some(Msg::EditorRedo),

        // 검색 네비게이션
        | KeyCode::Char('n') => Some(Msg::EditorSearchNext),
        | KeyCode::Char('N') => Some(Msg::EditorSearchPrev),

        // 기타
        | KeyCode::Esc => Some(Msg::EditorBack),
        | _ => None,
    }
}

fn handle_editor_insert_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        | KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
        | KeyCode::Char(c) => Some(Msg::EditorInsertChar(c)),
        | KeyCode::Backspace => Some(Msg::EditorBackspace),
        | KeyCode::Enter => Some(Msg::EditorNewLine),
        | _ => None,
    }
}

fn execute_command(cmd: update::Command, model: &mut Model) -> std::io::Result<()> {
    use update::Command;

    match cmd {
        | Command::LoadDiary(date) => match model.storage.load(date) {
            | Ok(content) => {
                update::update(model, Msg::LoadDiarySuccess(date, content));
            },
            | Err(e) => {
                update::update(model, Msg::LoadDiaryFailed(e.to_string()));
            },
        },
        | Command::SaveDiary(date, content) => match model.storage.save(date, &content) {
            | Ok(_) => {
                model.diary_entries.entries.insert(date);
                update::update(model, Msg::SaveDiarySuccess);
            },
            | Err(e) => {
                update::update(model, Msg::SaveDiaryFailed(e.to_string()));
            },
        },
        | Command::DeleteDiary(date) => match model.storage.delete(date) {
            | Ok(_) => {
                update::update(model, Msg::DeleteDiarySuccess(date));
            },
            | Err(e) => {
                update::update(model, Msg::SaveDiaryFailed(e.to_string()));
            },
        },
    }

    Ok(())
}
