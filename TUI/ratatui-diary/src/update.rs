use crate::model::{Model, Screen};
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
                // TODO: 커서 이동 구현
            }
        }
        Msg::CalendarMoveRight => {
            if model.screen == Screen::Calendar {
                // TODO: 커서 이동 구현
            }
        }
        Msg::CalendarMoveUp => {
            if model.screen == Screen::Calendar {
                // TODO: 커서 이동 구현
            }
        }
        Msg::CalendarMoveDown => {
            if model.screen == Screen::Calendar {
                // TODO: 커서 이동 구현
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

        _ => {}
    }

    None
}
