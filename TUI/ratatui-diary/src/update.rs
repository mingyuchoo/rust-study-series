use crate::{message::Msg,
            model::{EditorState,
                    EditorSubMode,
                    Model,
                    Screen,
                    Selection}};
use chrono::NaiveDate;

pub enum Command {
    LoadDiary(NaiveDate),
    SaveDiary(NaiveDate, String),
    DeleteDiary(NaiveDate),
}

/// Selection이 활성화되어 있으면 커서 이동시 selection도 업데이트
fn update_selection_on_move(state: &mut EditorState) {
    if let Some(ref mut sel) = state.selection {
        sel.cursor_line = state.cursor_line;
        sel.cursor_col = state.cursor_col;
    }
}

/// 클립보드 내용을 커서 위치에 붙여넣기
fn paste_clipboard(state: &mut EditorState) {
    if state.clipboard.is_empty() {
        return;
    }

    if state.clipboard.ends_with('\n') {
        // 줄 단위 붙여넣기
        let lines: Vec<String> = state.clipboard.trim_end_matches('\n').split('\n').map(String::from).collect();

        if state.content.is_empty() {
            state.content.push(String::new());
        }

        let insert_at = (state.cursor_line + 1).min(state.content.len());

        for (i, line) in lines.iter().enumerate() {
            state.content.insert(insert_at + i, line.clone());
        }

        state.cursor_line = insert_at;
        state.cursor_col = 0;
    } else {
        // 문자 단위 붙여넣기 — 커서 위치에 삽입
        if state.cursor_line >= state.content.len() {
            state.content.push(String::new());
        }

        let insert_byte_pos = state.char_idx_to_byte_idx(state.cursor_line, state.cursor_col);
        state.content[state.cursor_line].insert_str(insert_byte_pos, &state.clipboard);
        state.cursor_col += state.clipboard.chars().count();
    }

    state.is_modified = true;
}

pub fn update(model: &mut Model, msg: Msg) -> Option<Command> {
    match msg {
        | Msg::Quit => {
            // Handled by main loop
        },

        | Msg::DismissError => {
            model.show_error_popup = false;
            model.error_message = None;
        },

        // ===== 달력 네비게이션 =====
        | Msg::CalendarMoveLeft =>
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_left();
            },
        | Msg::CalendarMoveRight =>
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_right();
            },
        | Msg::CalendarMoveUp =>
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_up();
            },
        | Msg::CalendarMoveDown =>
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_down();
            },
        | Msg::CalendarSelectDate =>
            if model.screen == Screen::Calendar {
                let date = model.calendar_state.selected_date;
                model.screen = Screen::Editor;
                model.editor_state.date = date;
                return Some(Command::LoadDiary(date));
            },

        // ===== 달력 월/년 이동 (직접 호출) =====
        | Msg::CalendarNextMonth =>
            if model.screen == Screen::Calendar {
                model.calendar_state.next_month();
            },
        | Msg::CalendarPrevMonth =>
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_month();
            },
        | Msg::CalendarNextYear =>
            if model.screen == Screen::Calendar {
                model.calendar_state.next_year();
            },
        | Msg::CalendarPrevYear =>
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_year();
            },

        // ===== 에디터 네비게이션 =====
        | Msg::EditorMoveLeft =>
            if model.screen == Screen::Editor && model.editor_state.cursor_col > 0 {
                model.editor_state.cursor_col -= 1;
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorMoveRight =>
            if model.screen == Screen::Editor {
                let line_len = if model.editor_state.cursor_line < model.editor_state.content.len() {
                    model.editor_state.content[model.editor_state.cursor_line].chars().count()
                } else {
                    0
                };
                if model.editor_state.cursor_col < line_len {
                    model.editor_state.cursor_col += 1;
                    update_selection_on_move(&mut model.editor_state);
                }
            },
        | Msg::EditorMoveUp =>
            if model.screen == Screen::Editor && model.editor_state.cursor_line > 0 {
                model.editor_state.cursor_line -= 1;
                let line_len = model.editor_state.content[model.editor_state.cursor_line].chars().count();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorMoveDown =>
            if model.screen == Screen::Editor && model.editor_state.cursor_line + 1 < model.editor_state.content.len() {
                model.editor_state.cursor_line += 1;
                let line_len = model.editor_state.content[model.editor_state.cursor_line].chars().count();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorWordNext =>
            if model.screen == Screen::Editor {
                model.editor_state.move_word_next();
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorWordPrev =>
            if model.screen == Screen::Editor {
                model.editor_state.move_word_prev();
                update_selection_on_move(&mut model.editor_state);
            },

        // ===== 에디터 점프 (직접 호출, 서브모드 불필요) =====
        | Msg::EditorGotoDocStart =>
            if model.screen == Screen::Editor {
                model.editor_state.cursor_line = 0;
                model.editor_state.cursor_col = 0;
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorGotoDocEnd =>
            if model.screen == Screen::Editor && !model.editor_state.content.is_empty() {
                model.editor_state.cursor_line = model.editor_state.content.len() - 1;
                model.editor_state.cursor_col = model.editor_state.content[model.editor_state.cursor_line].chars().count();
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorGotoLineStart =>
            if model.screen == Screen::Editor {
                model.editor_state.cursor_col = 0;
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorGotoLineEnd =>
            if model.screen == Screen::Editor {
                let line_len = if model.editor_state.cursor_line < model.editor_state.content.len() {
                    model.editor_state.content[model.editor_state.cursor_line].chars().count()
                } else {
                    0
                };
                model.editor_state.cursor_col = line_len;
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorExitSubMode =>
            if model.screen == Screen::Editor {
                model.editor_state.submode = None;
            },

        // ===== 에디터 문자 입력 (항상 활성) =====
        | Msg::EditorInsertChar(c) =>
            if model.screen == Screen::Editor {
                model.editor_state.insert_char(c);
            },
        | Msg::EditorBackspace =>
            if model.screen == Screen::Editor {
                model.editor_state.backspace();
            },
        | Msg::EditorNewLine =>
            if model.screen == Screen::Editor {
                model.editor_state.new_line();
            },
        | Msg::EditorOpenLine =>
            if model.screen == Screen::Editor {
                model.editor_state.open_line();
            },

        // ===== 에디터 Selection =====
        | Msg::EditorToggleSelection =>
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                if state.selection.is_some() {
                    state.selection = None;
                } else {
                    state.selection = Some(Selection {
                        anchor_line: state.cursor_line,
                        anchor_col: state.cursor_col,
                        cursor_line: state.cursor_line,
                        cursor_col: state.cursor_col,
                    });
                }
            },
        | Msg::EditorSelectLine =>
            if model.screen == Screen::Editor {
                let state = &mut model.editor_state;
                let line_len = if state.cursor_line < state.content.len() {
                    state.content[state.cursor_line].chars().count()
                } else {
                    0
                };
                state.selection = Some(Selection {
                    anchor_line: state.cursor_line,
                    anchor_col: 0,
                    cursor_line: state.cursor_line,
                    cursor_col: line_len,
                });
            },

        // ===== 에디터 편집 기능 =====
        | Msg::EditorDelete => {
            if model.screen == Screen::Editor {
                // Selection이 있을 때만 삭제
                if model.editor_state.selection.is_some() {
                    if let Some(text) = model.editor_state.get_selected_text() {
                        model.editor_state.clipboard = text;
                    }
                    model.editor_state.delete_selection();
                    model.editor_state.save_snapshot();
                }
            }
        },
        | Msg::EditorDeleteForward =>
            if model.screen == Screen::Editor {
                model.editor_state.delete_forward();
                model.editor_state.save_snapshot();
            },
        | Msg::EditorKillLine =>
            if model.screen == Screen::Editor {
                let killed = model.editor_state.kill_line();
                if !killed.is_empty() {
                    model.editor_state.clipboard = killed;
                }
                model.editor_state.save_snapshot();
            },
        | Msg::EditorYank =>
            if model.screen == Screen::Editor
                && let Some(text) = model.editor_state.get_selected_text()
            {
                model.editor_state.clipboard = text;
                model.editor_state.selection = None;
            },
        | Msg::EditorPaste =>
            if model.screen == Screen::Editor {
                paste_clipboard(&mut model.editor_state);
                model.editor_state.save_snapshot();
            },

        // ===== 에디터 Undo/Redo =====
        | Msg::EditorUndo =>
            if model.screen == Screen::Editor {
                model.editor_state.undo();
            },
        | Msg::EditorRedo =>
            if model.screen == Screen::Editor {
                model.editor_state.redo();
            },

        // ===== 에디터 검색 =====
        | Msg::EditorEnterSearchMode =>
            if model.screen == Screen::Editor {
                model.editor_state.submode = Some(EditorSubMode::Search);
                model.editor_state.search_pattern.clear();
            },
        | Msg::EditorSearchChar(c) =>
            if model.screen == Screen::Editor && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.search_pattern.push(c);
            },
        | Msg::EditorSearchBackspace =>
            if model.screen == Screen::Editor && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.search_pattern.pop();
            },
        | Msg::EditorExecuteSearch =>
            if model.screen == Screen::Editor && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.execute_search();
                model.editor_state.submode = None;
            },
        | Msg::EditorSearchNext =>
            if model.screen == Screen::Editor {
                model.editor_state.search_next();
            },
        | Msg::EditorSearchPrev =>
            if model.screen == Screen::Editor {
                model.editor_state.search_prev();
            },

        // ===== 에디터 Ctrl+X 프리픽스 =====
        | Msg::EditorEnterCtrlXMode =>
            if model.screen == Screen::Editor {
                model.editor_state.submode = Some(EditorSubMode::CtrlX);
            },
        | Msg::EditorCtrlXSave =>
            if model.screen == Screen::Editor && model.editor_state.submode == Some(EditorSubMode::CtrlX) {
                let date = model.editor_state.date;
                let content = model.editor_state.get_content();
                model.editor_state.submode = None;
                return Some(Command::SaveDiary(date, content));
            },
        | Msg::EditorCtrlXBack =>
            if model.screen == Screen::Editor && model.editor_state.submode == Some(EditorSubMode::CtrlX) {
                model.screen = Screen::Calendar;
                model.editor_state.submode = None;
            },
        // ===== 파일 I/O 결과 =====
        | Msg::LoadDiarySuccess(date, content) =>
            if model.screen == Screen::Editor {
                model.editor_state.date = date;
                model.editor_state.load_content(&content);
            },
        | Msg::LoadDiaryFailed(error) =>
            if !error.contains("No such file") {
                model.error_message = Some(format!("로드 실패: {}", error));
                model.show_error_popup = true;
            },
        | Msg::SaveDiarySuccess => {
            model.editor_state.is_modified = false;
        },
        | Msg::SaveDiaryFailed(error) => {
            model.error_message = Some(format!("저장 실패: {}", error));
            model.show_error_popup = true;
        },
        | Msg::DeleteDiarySuccess(date) => {
            model.diary_entries.entries.remove(&date);
            model.screen = Screen::Calendar;
        },
        | Msg::RefreshIndex(entries) => {
            model.diary_entries.entries = entries;
        },
    }

    None
}
