use application::usecases::AddressUseCases;
use domain::entities::Address;
use iced::widget::{Column, button, column, container, row, scrollable, text, text_input};
use iced::{Element, Font, Length, Task};
use infrastructure::database::SqliteAddressRepository;
use std::sync::Arc;

const NOTO_SANS_KR: &[u8] = include_bytes!("../fonts/NotoSansKR-Regular.ttf");

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    PhoneChanged(String),
    EmailChanged(String),
    AddressChanged(String),
    CreateAddress,
    DeleteAddress(i64),
    EditAddress(Address),
    UpdateAddress,
    CancelEdit,
    LoadAddresses,
    AddressesLoaded(Result<Vec<Address>, String>),
}

struct AddressBook {
    usecases: Arc<AddressUseCases>,
    addresses: Vec<Address>,
    name_input: String,
    phone_input: String,
    email_input: String,
    address_input: String,
    editing_id: Option<i64>,
}

impl AddressBook {
    fn new() -> (Self, Task<Message>) {
        let repository = Arc::new(SqliteAddressRepository::new("addresses.db").expect("Failed to initialize database"));
        let usecases = Arc::new(AddressUseCases::new(repository));

        (
            Self {
                usecases,
                addresses: Vec::new(),
                name_input: String::new(),
                phone_input: String::new(),
                email_input: String::new(),
                address_input: String::new(),
                editing_id: None,
            },
            Task::done(Message::LoadAddresses),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            | Message::NameChanged(value) => {
                self.name_input = value;
                Task::none()
            },
            | Message::PhoneChanged(value) => {
                self.phone_input = value;
                Task::none()
            },
            | Message::EmailChanged(value) => {
                self.email_input = value;
                Task::none()
            },
            | Message::AddressChanged(value) => {
                self.address_input = value;
                Task::none()
            },
            | Message::CreateAddress => {
                let usecases = self.usecases.clone();
                let name = self.name_input.clone();
                let phone = self.phone_input.clone();
                let email = self.email_input.clone();
                let address = self.address_input.clone();

                self.name_input.clear();
                self.phone_input.clear();
                self.email_input.clear();
                self.address_input.clear();

                Task::perform(
                    async move {
                        usecases.create_address(name, phone, email, address)?;
                        usecases.get_all_addresses()
                    },
                    Message::AddressesLoaded,
                )
            },
            | Message::DeleteAddress(id) => {
                let usecases = self.usecases.clone();
                Task::perform(
                    async move {
                        usecases.delete_address(id)?;
                        usecases.get_all_addresses()
                    },
                    Message::AddressesLoaded,
                )
            },
            | Message::EditAddress(address) => {
                self.editing_id = address.id;
                self.name_input = address.name;
                self.phone_input = address.phone;
                self.email_input = address.email;
                self.address_input = address.address;
                Task::none()
            },
            | Message::UpdateAddress =>
                if let Some(id) = self.editing_id {
                    let usecases = self.usecases.clone();
                    let address = Address {
                        id: Some(id),
                        name: self.name_input.clone(),
                        phone: self.phone_input.clone(),
                        email: self.email_input.clone(),
                        address: self.address_input.clone(),
                    };

                    self.name_input.clear();
                    self.phone_input.clear();
                    self.email_input.clear();
                    self.address_input.clear();
                    self.editing_id = None;

                    Task::perform(
                        async move {
                            usecases.update_address(address)?;
                            usecases.get_all_addresses()
                        },
                        Message::AddressesLoaded,
                    )
                } else {
                    Task::none()
                },
            | Message::CancelEdit => {
                self.editing_id = None;
                self.name_input.clear();
                self.phone_input.clear();
                self.email_input.clear();
                self.address_input.clear();
                Task::none()
            },
            | Message::LoadAddresses => {
                let usecases = self.usecases.clone();
                Task::perform(async move { usecases.get_all_addresses() }, Message::AddressesLoaded)
            },
            | Message::AddressesLoaded(result) => {
                match result {
                    | Ok(addresses) => self.addresses = addresses,
                    | Err(_) => {},
                }
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input_form = column![
            text("이름:").size(16),
            text_input("이름을 입력하세요", &self.name_input).on_input(Message::NameChanged).padding(10),
            text("전화번호:").size(16),
            text_input("전화번호를 입력하세요", &self.phone_input)
                .on_input(Message::PhoneChanged)
                .padding(10),
            text("이메일:").size(16),
            text_input("이메일을 입력하세요", &self.email_input).on_input(Message::EmailChanged).padding(10),
            text("주소:").size(16),
            text_input("주소를 입력하세요", &self.address_input)
                .on_input(Message::AddressChanged)
                .padding(10),
        ]
        .spacing(10)
        .padding(20);

        let action_buttons = if self.editing_id.is_some() {
            row![
                button("수정").on_press(Message::UpdateAddress).padding(10),
                button("취소").on_press(Message::CancelEdit).padding(10),
            ]
            .spacing(10)
        } else {
            row![button("추가").on_press(Message::CreateAddress).padding(10)]
        };

        let address_list: Element<_> = self
            .addresses
            .iter()
            .fold(Column::new().spacing(10), |col, addr| {
                col.push(
                    container(
                        column![
                            text(format!("이름: {}", addr.name)).size(18),
                            text(format!("전화: {}", addr.phone)).size(14),
                            text(format!("이메일: {}", addr.email)).size(14),
                            text(format!("주소: {}", addr.address)).size(14),
                            row![
                                button("수정").on_press(Message::EditAddress(addr.clone())).padding(5),
                                button("삭제").on_press(Message::DeleteAddress(addr.id.unwrap())).padding(5),
                            ]
                            .spacing(10),
                        ]
                        .spacing(5)
                        .padding(10),
                    )
                    .padding(10)
                    .style(container::rounded_box),
                )
            })
            .into();

        let content = column![
            text("주소록").size(32),
            input_form,
            action_buttons,
            text("저장된 주소").size(24),
            scrollable(address_list).height(Length::Fill),
        ]
        .spacing(20)
        .padding(20);

        container(content).width(Length::Fill).height(Length::Fill).into()
    }
}

fn main() -> iced::Result {
    iced::application("주소록", AddressBook::update, AddressBook::view)
        .font(NOTO_SANS_KR)
        .default_font(Font::DEFAULT)
        .run_with(AddressBook::new)
}
