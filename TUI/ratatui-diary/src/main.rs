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
        | Screen::Calendar => handle_calendar_key(key),
        | Screen::Editor => handle_editor_key(key, &model.editor_state),
    }
}

fn handle_calendar_key(key: KeyEvent) -> Option<Msg> {
    let mods = key.modifiers;

    match key.code {
        // Ctrl+Q: 종료
        | KeyCode::Char('q') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::Quit),

        // Ctrl+B/F/P/N: 이동
        | KeyCode::Char('b') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::CalendarMoveLeft),
        | KeyCode::Char('f') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::CalendarMoveRight),
        | KeyCode::Char('p') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::CalendarMoveUp),
        | KeyCode::Char('n') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::CalendarMoveDown),

        // Enter: 편집
        | KeyCode::Enter => Some(Msg::CalendarSelectDate),

        // Alt+N / Alt+P: 다음/이전 달
        | KeyCode::Char('n') if mods.contains(KeyModifiers::ALT) => Some(Msg::CalendarNextMonth),
        | KeyCode::Char('p') if mods.contains(KeyModifiers::ALT) => Some(Msg::CalendarPrevMonth),

        // Alt+] / Alt+[: 다음/이전 년
        | KeyCode::Char(']') if mods.contains(KeyModifiers::ALT) => Some(Msg::CalendarNextYear),
        | KeyCode::Char('[') if mods.contains(KeyModifiers::ALT) => Some(Msg::CalendarPrevYear),

        | _ => None,
    }
}

fn handle_editor_key(key: KeyEvent, state: &ratatui_diary::model::EditorState) -> Option<Msg> {
    use ratatui_diary::model::EditorSubMode;

    // 서브모드 처리 우선
    match &state.submode {
        | Some(EditorSubMode::CtrlX) => {
            return match key.code {
                // Ctrl+S: 저장
                | KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Msg::EditorCtrlXSave),
                // Ctrl+C: 뒤로 (달력)
                | KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Msg::EditorCtrlXBack),
                // Esc: 취소
                | KeyCode::Esc => Some(Msg::EditorExitSubMode),
                | _ => Some(Msg::EditorExitSubMode),
            };
        },
        | Some(EditorSubMode::Search) => {
            return match key.code {
                | KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Msg::EditorSearchNext),
                | KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Msg::EditorSearchPrev),
                | KeyCode::Char(c) => Some(Msg::EditorSearchChar(c)),
                | KeyCode::Enter => Some(Msg::EditorExecuteSearch),
                | KeyCode::Esc => Some(Msg::EditorExitSubMode),
                | KeyCode::Backspace => Some(Msg::EditorSearchBackspace),
                | _ => None,
            };
        },
        | None => {},
    }

    let mods = key.modifiers;

    match key.code {
        // === Ctrl 조합 ===
        // Ctrl+F/B/N/P: 커서 이동
        | KeyCode::Char('f') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorMoveRight),
        | KeyCode::Char('b') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorMoveLeft),
        | KeyCode::Char('n') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorMoveDown),
        | KeyCode::Char('p') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorMoveUp),

        // Ctrl+A / Ctrl+E: 줄 시작/끝
        | KeyCode::Char('a') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorGotoLineStart),
        | KeyCode::Char('e') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorGotoLineEnd),

        // Ctrl+H: backspace
        | KeyCode::Char('h') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorBackspace),

        // Ctrl+O: 커서 위치에 새 줄 열기 (open-line)
        | KeyCode::Char('o') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorOpenLine),

        // Ctrl+D: 커서 앞 문자 삭제
        | KeyCode::Char('d') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorDeleteForward),

        // Ctrl+K: kill-line (줄 끝까지 삭제)
        | KeyCode::Char('k') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorKillLine),

        // Ctrl+Space: 마크 설정 (선택 토글)
        | KeyCode::Char(' ') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorToggleSelection),

        // Ctrl+W: 영역 잘라내기
        | KeyCode::Char('w') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorDelete),

        // Ctrl+Y: 붙여넣기 (yank)
        | KeyCode::Char('y') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorPaste),

        // Ctrl+Z: 실행취소
        | KeyCode::Char('z') if mods == KeyModifiers::CONTROL => Some(Msg::EditorUndo),

        // Ctrl+Shift+Z: 다시실행
        | KeyCode::Char('Z') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorRedo),

        // Ctrl+S: 검색 (서브모드 진입)
        | KeyCode::Char('s') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorEnterSearchMode),

        // Ctrl+X: 프리픽스 모드 진입
        | KeyCode::Char('x') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::EditorEnterCtrlXMode),

        // Ctrl+Q: 종료
        | KeyCode::Char('q') if mods.contains(KeyModifiers::CONTROL) => Some(Msg::Quit),

        // === Alt 조합 ===
        // Alt+F / Alt+B: 단어 이동
        | KeyCode::Char('f') if mods.contains(KeyModifiers::ALT) => Some(Msg::EditorWordNext),
        | KeyCode::Char('b') if mods.contains(KeyModifiers::ALT) => Some(Msg::EditorWordPrev),

        // Alt+< / Alt+>: 문서 시작/끝
        | KeyCode::Char('<') if mods.contains(KeyModifiers::ALT) => Some(Msg::EditorGotoDocStart),
        | KeyCode::Char('>') if mods.contains(KeyModifiers::ALT) => Some(Msg::EditorGotoDocEnd),

        // Alt+W: 영역 복사
        | KeyCode::Char('w') if mods.contains(KeyModifiers::ALT) => Some(Msg::EditorYank),

        // === 일반 키 ===
        | KeyCode::Backspace => Some(Msg::EditorBackspace),
        | KeyCode::Enter => Some(Msg::EditorNewLine),
        | KeyCode::Char(c) => Some(Msg::EditorInsertChar(c)),

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
