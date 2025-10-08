use iced::widget::{Column, button, column, text};

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn view(&self) -> Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
    }

    fn update(&mut self, message: Message) {
        match message {
            | Message::Increment => {
                self.value += 1;
            },
            | Message::Decrement => {
                self.value -= 1;
            },
        }
    }
}

pub fn main() -> iced::Result { iced::run("A cool counter", Counter::update, Counter::view) }
