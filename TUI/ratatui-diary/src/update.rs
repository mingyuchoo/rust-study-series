use crate::model::{Model, Screen, EditorMode, EditorState};
use crate::message::Msg;
use chrono::NaiveDate;

pub enum Command {
    LoadDiary(NaiveDate),
    SaveDiary(NaiveDate, String),
    DeleteDiary(NaiveDate),
}

pub fn update(model: &mut Model, msg: Msg) -> Option<Command> {
    match msg {
        Msg::Quit => {
            // Handled by main loop
        }

        Msg::DismissError => {
            model.show_error_popup = false;
            model.error_message = None;
        }

        // 달력 네비게이션
        Msg::CalendarMoveLeft => {
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_left();
            }
        }
        Msg::CalendarMoveRight => {
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_right();
            }
        }
        Msg::CalendarMoveUp => {
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_up();
            }
        }
        Msg::CalendarMoveDown => {
            if model.screen == Screen::Calendar {
                model.calendar_state.move_cursor_down();
            }
        }
        Msg::CalendarNextMonth => {
            if model.screen == Screen::Calendar {
                model.calendar_state.next_month();
            }
        }
        Msg::CalendarPrevMonth => {
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_month();
            }
        }
        Msg::CalendarNextYear => {
            if model.screen == Screen::Calendar {
                model.calendar_state.next_year();
            }
        }
        Msg::CalendarPrevYear => {
            if model.screen == Screen::Calendar {
                model.calendar_state.prev_year();
            }
        }
        Msg::CalendarSelectDate => {
            if model.screen == Screen::Calendar {
                let date = model.calendar_state.selected_date;
                model.screen = Screen::Editor;
                model.editor_state.date = date;
                return Some(Command::LoadDiary(date));
            }
        }

        // 에디터 - Normal 모드
        Msg::EditorEnterInsertMode => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                model.editor_state.mode = EditorMode::Insert;
            }
        }
        Msg::EditorEnterNormalMode => {
            if model.screen == Screen::Editor {
                model.editor_state.mode = EditorMode::Normal;
            }
        }
        Msg::EditorDeleteLine => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                let date = model.editor_state.date;
                return Some(Command::DeleteDiary(date));
            }
        }
        Msg::EditorStartCommand => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Normal {
                // TODO: Helix 스타일로 재작성 예정
                // model.editor_state.mode = EditorMode::Command(String::new());
            }
        }

        // 에디터 - Insert 모드
        Msg::EditorInsertChar(c) => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.insert_char(c);
            }
        }
        Msg::EditorBackspace => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.backspace();
            }
        }
        Msg::EditorNewLine => {
            if model.screen == Screen::Editor && model.editor_state.mode == EditorMode::Insert {
                model.editor_state.new_line();
            }
        }

        // 에디터 - Command 모드
        // TODO: Helix 스타일로 재작성 예정
        Msg::EditorCommandChar(_c) => {
            // if let EditorMode::Command(ref mut cmd) = model.editor_state.mode {
            //     cmd.push(c);
            // }
        }
        Msg::EditorExecuteCommand => {
            // if let EditorMode::Command(ref cmd) = model.editor_state.mode.clone() {
            //     let date = model.editor_state.date;
            //     let content = model.editor_state.get_content();

            //     match cmd.as_str() {
            //         "w" => {
            //             model.editor_state.mode = EditorMode::Normal;
            //             return Some(Command::SaveDiary(date, content));
            //         }
            //         "q" => {
            //             model.screen = Screen::Calendar;
            //             model.editor_state = EditorState::new(date);
            //         }
            //         "wq" => {
            //             model.screen = Screen::Calendar;
            //             let old_state = std::mem::replace(
            //                 &mut model.editor_state,
            //                 EditorState::new(date)
            //             );
            //             return Some(Command::SaveDiary(date, old_state.get_content()));
            //         }
            //         _ => {
            //             model.error_message = Some(format!("Unknown command: {}", cmd));
            //             model.show_error_popup = true;
            //         }
            //     }
            //     model.editor_state.mode = EditorMode::Normal;
            // }
        }
        Msg::EditorBack => {
            if model.screen == Screen::Editor {
                model.screen = Screen::Calendar;
            }
        }

        // 파일 I/O 결과
        Msg::LoadDiarySuccess(date, content) => {
            if model.screen == Screen::Editor {
                model.editor_state.date = date;
                model.editor_state.load_content(&content);
            }
        }
        Msg::LoadDiaryFailed(error) => {
            // 파일 없음 = 새 다이어리, 에러 표시 안함
            if !error.contains("No such file") {
                model.error_message = Some(format!("로드 실패: {}", error));
                model.show_error_popup = true;
            }
        }
        Msg::SaveDiarySuccess => {
            model.editor_state.is_modified = false;
        }
        Msg::SaveDiaryFailed(error) => {
            model.error_message = Some(format!("저장 실패: {}", error));
            model.show_error_popup = true;
        }
        Msg::DeleteDiarySuccess(date) => {
            model.diary_entries.entries.remove(&date);
            model.screen = Screen::Calendar;
        }
        Msg::RefreshIndex(entries) => {
            model.diary_entries.entries = entries;
        }

        _ => {}
    }

    None
}
