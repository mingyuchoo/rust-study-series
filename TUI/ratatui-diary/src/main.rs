use crossterm::{event::{self,
                        Event,
                        KeyCode,
                        KeyEvent,
                        KeyModifiers},
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
                    update,
                    view,
                    storage::Storage};
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
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Some(msg) = handle_key(key, model) {
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
        | Screen::Calendar => handle_calendar_key(key),
        | Screen::Editor => handle_editor_key(key, &model.editor_state.mode),
    }
}

fn handle_calendar_key(key: KeyEvent) -> Option<Msg> {
    match (key.code, key.modifiers) {
        | (KeyCode::Char('q'), _) => Some(Msg::Quit),
        | (KeyCode::Char('h'), _) => Some(Msg::CalendarMoveLeft),
        | (KeyCode::Char('l'), _) => Some(Msg::CalendarMoveRight),
        | (KeyCode::Char('j'), _) => Some(Msg::CalendarMoveDown),
        | (KeyCode::Char('k'), _) => Some(Msg::CalendarMoveUp),
        | (KeyCode::Char('H'), KeyModifiers::SHIFT) => Some(Msg::CalendarPrevYear),
        | (KeyCode::Char('L'), KeyModifiers::SHIFT) => Some(Msg::CalendarNextYear),
        | (KeyCode::Enter, _) => Some(Msg::CalendarSelectDate),
        | _ => None,
    }
}

fn handle_editor_key(key: KeyEvent, mode: &ratatui_diary::model::EditorMode) -> Option<Msg> {
    use ratatui_diary::model::EditorMode;

    match mode {
        | EditorMode::Normal => match key.code {
            | KeyCode::Char('i') => Some(Msg::EditorEnterInsertMode),
            | KeyCode::Char(':') => Some(Msg::EditorStartCommand),
            | KeyCode::Char('d') => Some(Msg::EditorDeleteLine), // dd는 두 번 누르기
            | KeyCode::Esc => Some(Msg::EditorBack),
            | _ => None,
        },
        | EditorMode::Insert => match key.code {
            | KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
            | KeyCode::Char(c) => Some(Msg::EditorInsertChar(c)),
            | KeyCode::Backspace => Some(Msg::EditorBackspace),
            | KeyCode::Enter => Some(Msg::EditorNewLine),
            | _ => None,
        },
        | EditorMode::Command(_) => match key.code {
            | KeyCode::Char(c) => Some(Msg::EditorCommandChar(c)),
            | KeyCode::Enter => Some(Msg::EditorExecuteCommand),
            | KeyCode::Esc => Some(Msg::EditorEnterNormalMode),
            | KeyCode::Backspace => Some(Msg::EditorCommandChar('\x08')), // TODO: proper backspace
            | _ => None,
        },
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
