use crate::{message::{InsertPosition,
                      Msg},
            model::{CalendarSubMode,
                    EditorMode,
                    EditorState,
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

/// Helper function: Selection이 활성화되어 있으면 커서 이동시 selection도
/// 업데이트
fn update_selection_on_move(state: &mut EditorState) {
    if let Some(ref mut sel) = state.selection {
        sel.cursor_line = state.cursor_line;
        sel.cursor_col = state.cursor_col;
    }
}

/// Helper function: 선택 영역이 없으면 현재 줄을 선택 (Helix 스타일)
fn ensure_selection_for_edit(state: &mut EditorState) {
    if state.selection.is_none() {
        // 현재 줄 선택
        let line_len = if state.cursor_line < state.content.len() {
            state.content[state.cursor_line].len()
        } else {
            0
        };
        state.selection = Some(Selection {
            anchor_line: state.cursor_line,
            anchor_col: 0,
            cursor_line: state.cursor_line,
            cursor_col: line_len,
        });
    }
}

/// Helper function: 클립보드 내용을 붙여넣기 (줄 단위/문자 단위 모두 지원)
fn paste_clipboard(state: &mut EditorState, before: bool) {
    if state.clipboard.is_empty() {
        return;
    }

    // 클립보드가 줄 단위인지 확인 (\n으로 끝나는지)
    if state.clipboard.ends_with('\n') {
        // 줄 단위 붙여넣기
        let lines: Vec<String> = state.clipboard.trim_end_matches('\n').split('\n').map(String::from).collect();

        // content가 비어있으면 초기화
        if state.content.is_empty() {
            state.content.push(String::new());
        }

        let insert_at = if before {
            state.cursor_line.min(state.content.len())
        } else {
            (state.cursor_line + 1).min(state.content.len())
        };

        for (i, line) in lines.iter().enumerate() {
            state.content.insert(insert_at + i, line.clone());
        }

        state.cursor_line = insert_at;
        state.cursor_col = 0;
    } else {
        // 문자 단위 붙여넣기
        // 안전한 접근: cursor_line이 범위를 벗어나면 새 줄 추가
        if state.cursor_line >= state.content.len() {
            state.content.push(String::new());
        }
        let line = &mut state.content[state.cursor_line];
        let insert_pos = if before { state.cursor_col } else { (state.cursor_col + 1).min(line.len()) };

        line.insert_str(insert_pos, &state.clipboard);
        state.cursor_col = insert_pos + state.clipboard.len();
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

        // ===== 달력 Space 모드 =====
        | Msg::CalendarEnterSpaceMode =>
            if model.screen == Screen::Calendar {
                model.calendar_state.submode = Some(CalendarSubMode::Space);
            },
        | Msg::CalendarExitSubMode =>
            if model.screen == Screen::Calendar {
                model.calendar_state.submode = None;
            },
        | Msg::CalendarSpaceQuit => {
            if model.screen == Screen::Calendar && model.calendar_state.submode == Some(CalendarSubMode::Space) {
                // Quit handled by main loop
                model.calendar_state.submode = None;
            }
        },
        | Msg::CalendarSpaceNextMonth =>
            if model.screen == Screen::Calendar && model.calendar_state.submode == Some(CalendarSubMode::Space) {
                model.calendar_state.next_month();
                model.calendar_state.submode = None;
            },
        | Msg::CalendarSpacePrevMonth =>
            if model.screen == Screen::Calendar && model.calendar_state.submode == Some(CalendarSubMode::Space) {
                model.calendar_state.prev_month();
                model.calendar_state.submode = None;
            },
        | Msg::CalendarSpaceNextYear =>
            if model.screen == Screen::Calendar && model.calendar_state.submode == Some(CalendarSubMode::Space) {
                model.calendar_state.next_year();
                model.calendar_state.submode = None;
            },
        | Msg::CalendarSpacePrevYear =>
            if model.screen == Screen::Calendar && model.calendar_state.submode == Some(CalendarSubMode::Space) {
                model.calendar_state.prev_year();
                model.calendar_state.submode = None;
            },

        // ===== 에디터 네비게이션 (Normal 모드) =====
        | Msg::EditorMoveLeft =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.cursor_col > 0 {
                model.editor_state.cursor_col -= 1;
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorMoveRight =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                let line_len = if model.editor_state.cursor_line < model.editor_state.content.len() {
                    model.editor_state.content[model.editor_state.cursor_line].len()
                } else {
                    0
                };
                if model.editor_state.cursor_col < line_len {
                    model.editor_state.cursor_col += 1;
                    update_selection_on_move(&mut model.editor_state);
                }
            },
        | Msg::EditorMoveUp => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.cursor_line > 0 {
                model.editor_state.cursor_line -= 1;
                // Clamp cursor_col to new line length
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            }
        },
        | Msg::EditorMoveDown => {
            if model.screen == Screen::Editor
                && model.editor_state.mode == EditorMode::Normal
                && model.editor_state.cursor_line + 1 < model.editor_state.content.len()
            {
                model.editor_state.cursor_line += 1;
                // Clamp cursor_col to new line length
                let line_len = model.editor_state.content[model.editor_state.cursor_line].len();
                model.editor_state.cursor_col = model.editor_state.cursor_col.min(line_len);
                update_selection_on_move(&mut model.editor_state);
            }
        },
        | Msg::EditorWordNext =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.move_word_next();
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorWordPrev =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.move_word_prev();
                update_selection_on_move(&mut model.editor_state);
            },
        | Msg::EditorWordEnd =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.move_word_end();
                update_selection_on_move(&mut model.editor_state);
            },

        // ===== 에디터 Goto 모드 =====
        | Msg::EditorEnterGotoMode =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.submode = Some(EditorSubMode::Goto);
            },
        | Msg::EditorGotoDocStart => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Goto) {
                model.editor_state.cursor_line = 0;
                model.editor_state.cursor_col = 0;
                update_selection_on_move(&mut model.editor_state);
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorGotoDocEnd => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Goto) {
                if !model.editor_state.content.is_empty() {
                    model.editor_state.cursor_line = model.editor_state.content.len() - 1;
                    model.editor_state.cursor_col = model.editor_state.content[model.editor_state.cursor_line].len();
                }
                update_selection_on_move(&mut model.editor_state);
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorGotoLineStart => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Goto) {
                model.editor_state.cursor_col = 0;
                update_selection_on_move(&mut model.editor_state);
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorGotoLineEnd => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Goto) {
                let line_len = if model.editor_state.cursor_line < model.editor_state.content.len() {
                    model.editor_state.content[model.editor_state.cursor_line].len()
                } else {
                    0
                };
                model.editor_state.cursor_col = line_len;
                update_selection_on_move(&mut model.editor_state);
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorExitSubMode =>
            if model.screen == Screen::Editor {
                model.editor_state.submode = None;
            },

        // ===== 에디터 Insert 모드 =====
        | Msg::EditorEnterInsert(pos) => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.mode = EditorMode::Insert;
                match pos {
                    | InsertPosition::BeforeCursor => {
                        // 커서 위치 유지
                    },
                    | InsertPosition::AfterCursor => {
                        let line_len = if model.editor_state.cursor_line < model.editor_state.content.len() {
                            model.editor_state.content[model.editor_state.cursor_line].len()
                        } else {
                            0
                        };
                        if model.editor_state.cursor_col < line_len {
                            model.editor_state.cursor_col += 1;
                        }
                    },
                    | InsertPosition::LineBelow =>
                        if model.editor_state.cursor_line < model.editor_state.content.len() {
                            model.editor_state.cursor_line += 1;
                            model.editor_state.content.insert(model.editor_state.cursor_line, String::new());
                            model.editor_state.cursor_col = 0;
                        },
                    | InsertPosition::LineAbove => {
                        model.editor_state.content.insert(model.editor_state.cursor_line, String::new());
                        model.editor_state.cursor_col = 0;
                    },
                }
                // Insert 모드 진입시 selection 해제
                model.editor_state.selection = None;
            }
        },
        | Msg::EditorEnterNormalMode => {
            if model.screen == Screen::Editor {
                if model.editor_state.mode == EditorMode::Insert {
                    // Insert 모드에서 나갈 때 스냅샷 저장
                    model.editor_state.save_snapshot();
                }
                model.editor_state.mode = EditorMode::Normal;
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorInsertChar(c) =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.insert_char(c);
            },
        | Msg::EditorBackspace =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.backspace();
            },
        | Msg::EditorNewLine =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.new_line();
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
                    state.content[state.cursor_line].len()
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
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                // 선택 영역이 없으면 현재 줄 선택 (Helix 스타일)
                ensure_selection_for_edit(&mut model.editor_state);

                // 스냅샷 저장 전에 선택 영역 복사
                if let Some(text) = model.editor_state.get_selected_text() {
                    model.editor_state.clipboard = text;
                }
                model.editor_state.delete_selection();
                model.editor_state.save_snapshot();
            }
        },
        | Msg::EditorChange => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                // 선택 영역이 없으면 현재 줄 선택 (Helix 스타일)
                ensure_selection_for_edit(&mut model.editor_state);

                // Delete 후 Insert 모드 진입
                if let Some(text) = model.editor_state.get_selected_text() {
                    model.editor_state.clipboard = text;
                }
                model.editor_state.delete_selection();
                model.editor_state.mode = EditorMode::Insert;
                model.editor_state.save_snapshot();
            }
        },
        | Msg::EditorYank => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                // 선택 영역이 없으면 현재 줄 선택 (Helix 스타일)
                ensure_selection_for_edit(&mut model.editor_state);

                if let Some(text) = model.editor_state.get_selected_text() {
                    model.editor_state.clipboard = text;
                    model.editor_state.selection = None;
                }
            }
        },
        | Msg::EditorPasteAfter =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                paste_clipboard(&mut model.editor_state, false);
                model.editor_state.save_snapshot();
            },
        | Msg::EditorPasteBefore =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                paste_clipboard(&mut model.editor_state, true);
                model.editor_state.save_snapshot();
            },

        // ===== 에디터 Undo/Redo =====
        | Msg::EditorUndo =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.undo();
            },
        | Msg::EditorRedo =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.redo();
            },

        // ===== 에디터 검색 =====
        | Msg::EditorEnterSearchMode =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.submode = Some(EditorSubMode::Search);
                model.editor_state.search_pattern.clear();
            },
        | Msg::EditorSearchChar(c) => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.search_pattern.push(c);
            }
        },
        | Msg::EditorSearchBackspace => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.search_pattern.pop();
            }
        },
        | Msg::EditorExecuteSearch => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal && model.editor_state.submode == Some(EditorSubMode::Search) {
                model.editor_state.execute_search();
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorSearchNext =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.search_next();
            },
        | Msg::EditorSearchPrev =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.search_prev();
            },

        // ===== 에디터 Space 명령 =====
        | Msg::EditorEnterSpaceMode =>
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.submode = Some(EditorSubMode::SpaceCommand);
            },
        | Msg::EditorSpaceSave => {
            if model.screen == Screen::Editor
                && model.editor_state.mode == EditorMode::Normal
                && model.editor_state.submode == Some(EditorSubMode::SpaceCommand)
            {
                let date = model.editor_state.date;
                let content = model.editor_state.get_content();
                model.editor_state.submode = None;
                return Some(Command::SaveDiary(date, content));
            }
        },
        | Msg::EditorSpaceQuit => {
            if model.screen == Screen::Editor
                && model.editor_state.mode == EditorMode::Normal
                && model.editor_state.submode == Some(EditorSubMode::SpaceCommand)
            {
                model.screen = Screen::Calendar;
                model.editor_state.submode = None;
            }
        },
        | Msg::EditorSpaceSaveQuit => {
            if model.screen == Screen::Editor
                && model.editor_state.mode == EditorMode::Normal
                && model.editor_state.submode == Some(EditorSubMode::SpaceCommand)
            {
                let date = model.editor_state.date;
                let content = model.editor_state.get_content();
                model.screen = Screen::Calendar;
                model.editor_state.submode = None;
                return Some(Command::SaveDiary(date, content));
            }
        },
        | Msg::EditorBack =>
            if model.screen == Screen::Editor {
                model.screen = Screen::Calendar;
            },

        // ===== 파일 I/O 결과 =====
        | Msg::LoadDiarySuccess(date, content) =>
            if model.screen == Screen::Editor {
                model.editor_state.date = date;
                model.editor_state.load_content(&content);
            },
        | Msg::LoadDiaryFailed(error) => {
            // 파일 없음 = 새 다이어리, 에러 표시 안함
            if !error.contains("No such file") {
                model.error_message = Some(format!("로드 실패: {}", error));
                model.show_error_popup = true;
            }
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
