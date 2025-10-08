use anyhow::Error;
use egui::{Color32, RichText};
use faker_rand::en_us::names::FirstName;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::mpsc::{Receiver, Sender, channel};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Record, Root};
use surrealdb::{RecordId, RecordIdKey, Surreal};

const PERSON: &str = "person";

#[derive(Serialize, Deserialize)]
struct Params<'a> {
    name: &'a str,
    pass: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PersonData {
    name: String,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    id: RecordId,
    created_by: Option<RecordId>,
}

#[derive(Debug, Clone)]
enum Command {
    CreatePerson(String),
    DeletePerson(String),
    ListPeople,
    RawQuery(String),
    SignUp,
    SignIn(String),
    SignInRoot,
    Session,
}

#[derive(Debug, Clone, PartialEq)]
enum AppTab {
    People,
    Authentication,
    Query,
    Session,
}

#[derive(Debug, Clone)]
enum MessageType {
    Success,
    Error,
    #[allow(dead_code)]
    Info,
}

#[derive(Debug, Clone)]
struct AppMessage {
    content: String,
    msg_type: MessageType,
    timestamp: std::time::Instant,
}

struct Database {
    client: Surreal<Client>,
    command_receiver: Receiver<Command>,
    response_sender: Sender<String>,
}

impl Deref for Database {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target { &self.client }
}

trait StringIt {
    fn string(self) -> Result<String, Error>;
}

impl StringIt for Option<Person> {
    fn string(self) -> Result<String, Error> {
        match self {
            | Some(t) => Ok(format!("{t:?}")),
            | None => Ok("[]".into()),
        }
    }
}

impl Database {
    async fn handle_command(&self, command: Command) -> Result<String, Error> {
        match command {
            | Command::CreatePerson(s) => {
                let person_data: PersonData = serde_json::from_str(&s)?;
                self.create::<Option<Person>>(PERSON).content(person_data).await?.string()
            },
            | Command::DeletePerson(s) =>
                if s.is_empty() {
                    let res: Vec<Person> = self.delete(PERSON).await?;
                    Ok(format!("{res:?}"))
                } else {
                    let key = RecordIdKey::from(s);
                    self.delete::<Option<Person>>((PERSON, key)).await?.string()
                },
            | Command::ListPeople => {
                let person: Vec<Person> = self.select(PERSON).await?;
                Ok(format!("{person:?}"))
            },
            | Command::SignUp => {
                let name = rand::random::<FirstName>().to_string();
                let pass = rand::random::<FirstName>().to_string();
                self.signup(Record {
                    access: "account",
                    namespace: "test",
                    database: "test",
                    params: Params {
                        name: &name,
                        pass: &pass,
                    },
                })
                .await?;
                Ok(format!(
                    "New user created!\n\n{{ \"name\": \"{name}\", \n \
                            \"pass\": \"{pass}\" }}"
                ))
            },
            | Command::RawQuery(q) => match self.query(q).await {
                | Ok(ok) => Ok(format!("{ok:?}")),
                | Err(e) => Ok(e.to_string()),
            },
            | Command::SignIn(s) => {
                let Ok(Params {
                    name,
                    pass,
                }) = serde_json::from_str::<Params>(&s)
                else {
                    return Ok("Params don't work!".to_string());
                };
                self.signin(Record {
                    access: "account",
                    namespace: "test",
                    database: "test",
                    params: Params {
                        name,
                        pass,
                    },
                })
                .await?;
                Ok(format!("Signed in as {name}!"))
            },
            | Command::SignInRoot => {
                self.signin(Root {
                    username: "root",
                    password: "root",
                })
                .await?;
                Ok("Back to root!".to_string())
            },
            | Command::Session => Ok(self
                .query("RETURN <string>$session")
                .await?
                .take::<Option<String>>(0)?
                .unwrap_or("No session data found!".into())),
        }
    }
}

struct SurrealDbApp {
    // UI State
    current_tab: AppTab,

    // Person Management
    person_name: String,
    person_id_to_delete: String,
    people_list: String,

    // Authentication
    auth_username: String,
    auth_password: String,
    current_user: String,

    // Query
    raw_query: String,
    query_result: String,

    // Session
    session_info: String,

    // Messages and Status
    messages: Vec<AppMessage>,
    is_loading: bool,
    connection_status: String,

    // Communication
    command_sender: Sender<Command>,
    response_receiver: Receiver<String>,
}

impl SurrealDbApp {
    fn send(&mut self, command: Command) {
        if let Err(e) = self.command_sender.send(command) {
            self.add_message(e.to_string(), MessageType::Error);
        } else {
            self.is_loading = true;
        }
    }

    fn add_message(&mut self, content: String, msg_type: MessageType) {
        self.messages.push(AppMessage {
            content,
            msg_type,
            timestamp: std::time::Instant::now(),
        });

        // Keep only last 10 messages
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
    }

    fn show_messages(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.label(RichText::new("Messages").heading().color(Color32::WHITE));

        for message in &self.messages {
            let color = match message.msg_type {
                | MessageType::Success => Color32::from_rgb(0, 200, 0),
                | MessageType::Error => Color32::from_rgb(200, 0, 0),
                | MessageType::Info => Color32::from_rgb(100, 150, 255),
            };

            let elapsed = message.timestamp.elapsed().as_secs();
            ui.horizontal(|ui| {
                ui.label(RichText::new(&message.content).color(color));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("{}s ago", elapsed)).small().color(Color32::GRAY));
                });
            });
        }
    }

    fn show_people_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("People Management");
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.label(RichText::new("Create New Person").strong());
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.person_name);
                    });

                    let create_enabled = !self.person_name.trim().is_empty() && !self.is_loading;
                    if ui.add_enabled(create_enabled, egui::Button::new("+ Create Person")).clicked() {
                        let person_data = format!(r#"{{"name": "{}"}}"#, self.person_name.trim());
                        self.send(Command::CreatePerson(person_data));
                        self.person_name.clear();
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(RichText::new("Delete Person").strong());
                    ui.horizontal(|ui| {
                        ui.label("ID:");
                        ui.text_edit_singleline(&mut self.person_id_to_delete);
                    });
                    ui.label(RichText::new("Tip: Leave empty to delete all people").small().color(Color32::GRAY));

                    if ui.add_enabled(!self.is_loading, egui::Button::new("Delete")).clicked() {
                        self.send(Command::DeletePerson(self.person_id_to_delete.clone()));
                    }
                });

                ui.add_space(10.0);

                if ui.add_enabled(!self.is_loading, egui::Button::new("List All People")).clicked() {
                    self.send(Command::ListPeople);
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.label(RichText::new("People List").strong());
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    if self.people_list.is_empty() {
                        ui.label(RichText::new("No people loaded. Click 'List All People' to refresh.").color(Color32::GRAY));
                    } else {
                        ui.text_edit_multiline(&mut self.people_list);
                    }
                });
            });
        });
    }

    fn show_auth_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Authentication");
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.label(RichText::new("Create New User").strong());
                    if ui.add_enabled(!self.is_loading, egui::Button::new("Generate Random User")).clicked() {
                        self.send(Command::SignUp);
                    }
                    ui.label(RichText::new("Tip: Creates a user with random credentials").small().color(Color32::GRAY));
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(RichText::new("Sign In as User").strong());
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut self.auth_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.text_edit_singleline(&mut self.auth_password);
                    });

                    let signin_enabled = !self.auth_username.trim().is_empty() && !self.auth_password.trim().is_empty() && !self.is_loading;

                    if ui.add_enabled(signin_enabled, egui::Button::new("Sign In")).clicked() {
                        let auth_data = format!(r#"{{"name": "{}", "pass": "{}"}}"#, self.auth_username.trim(), self.auth_password.trim());
                        self.send(Command::SignIn(auth_data));
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(RichText::new("Admin Access").strong());
                    if ui.add_enabled(!self.is_loading, egui::Button::new("Sign In as Root")).clicked() {
                        self.send(Command::SignInRoot);
                    }
                    ui.label(RichText::new("Tip: Switch to root user for admin operations").small().color(Color32::GRAY));
                });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.label(RichText::new("Current User").strong());
                if self.current_user.is_empty() {
                    ui.label(RichText::new("Not signed in").color(Color32::GRAY));
                } else {
                    ui.label(RichText::new(&self.current_user).color(Color32::from_rgb(0, 200, 0)));
                }
            });
        });
    }

    fn show_query_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Raw Query Interface");
        ui.separator();

        ui.vertical(|ui| {
            ui.label(RichText::new("SurrealQL Query").strong());
            ui.text_edit_multiline(&mut self.raw_query);

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(!self.is_loading && !self.raw_query.trim().is_empty(), egui::Button::new("Execute Query"))
                    .clicked()
                {
                    self.send(Command::RawQuery(self.raw_query.clone()));
                }

                if ui.button("Clear").clicked() {
                    self.raw_query.clear();
                    self.query_result.clear();
                }
            });

            ui.add_space(10.0);
            ui.separator();

            ui.label(RichText::new("Query Result").strong());
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                if self.query_result.is_empty() {
                    ui.label(RichText::new("No query executed yet").color(Color32::GRAY));
                } else {
                    ui.text_edit_multiline(&mut self.query_result);
                }
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(RichText::new("Query Examples").strong());
                ui.label("• SELECT * FROM person;");
                ui.label("• CREATE person SET name = 'John Doe';");
                ui.label("• UPDATE person:abc SET name = 'Jane Doe';");
                ui.label("• DELETE person WHERE name = 'John';");
            });
        });
    }

    fn show_session_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Session Information");
        ui.separator();

        ui.vertical(|ui| {
            if ui.add_enabled(!self.is_loading, egui::Button::new("Refresh Session Data")).clicked() {
                self.send(Command::Session);
            }

            ui.add_space(10.0);

            ui.label(RichText::new("Session Details").strong());
            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                if self.session_info.is_empty() {
                    ui.label(RichText::new("No session data loaded. Click 'Refresh Session Data' to load.").color(Color32::GRAY));
                } else {
                    ui.text_edit_multiline(&mut self.session_info);
                }
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(RichText::new("About Sessions").strong());
                ui.label("Session data shows your current authentication state,");
                ui.label("including user information and permissions.");
                ui.label("This is useful for debugging authentication issues.");
            });
        });
    }
}

impl eframe::App for SurrealDbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle responses
        if let Ok(response) = self.response_receiver.try_recv() {
            self.is_loading = false;

            // Determine message type based on response content
            let msg_type = if response.contains("Error") || response.contains("error") {
                MessageType::Error
            } else {
                MessageType::Success
            };

            // Route response to appropriate field
            match self.current_tab {
                | AppTab::People =>
                    if response.contains("Person") || response.contains("person") {
                        self.people_list = response.clone();
                    },
                | AppTab::Authentication =>
                    if response.contains("Signed in") {
                        self.current_user = response.clone();
                    },
                | AppTab::Query => {
                    self.query_result = response.clone();
                },
                | AppTab::Session => {
                    self.session_info = response.clone();
                },
            }

            self.add_message(response, msg_type);
        }

        // Top panel with tabs and status
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SurrealDB Manager");

                ui.separator();

                // Tab buttons
                if ui.selectable_label(self.current_tab == AppTab::Session, "Session").clicked() {
                    self.current_tab = AppTab::Session;
                }
                if ui.selectable_label(self.current_tab == AppTab::Authentication, "Auth").clicked() {
                    self.current_tab = AppTab::Authentication;
                }
                if ui.selectable_label(self.current_tab == AppTab::People, "People").clicked() {
                    self.current_tab = AppTab::People;
                }
                if ui.selectable_label(self.current_tab == AppTab::Query, "Query").clicked() {
                    self.current_tab = AppTab::Query;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Connection status
                    let status_color = if self.connection_status.contains("Connected") {
                        Color32::from_rgb(0, 200, 0)
                    } else {
                        Color32::from_rgb(200, 0, 0)
                    };
                    ui.label(RichText::new(&self.connection_status).color(status_color));

                    if self.is_loading {
                        ui.spinner();
                    }
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| match self.current_tab {
            | AppTab::People => self.show_people_tab(ui),
            | AppTab::Authentication => self.show_auth_tab(ui),
            | AppTab::Query => self.show_query_tab(ui),
            | AppTab::Session => self.show_session_tab(ui),
        });

        // Bottom panel for messages
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.show_messages(ui);
        });
    }
}

fn main() -> Result<(), Error> {
    let (command_sender, command_receiver) = channel();
    let (response_sender, response_receiver) = channel();

    std::thread::spawn(|| -> Result<(), Error> {
        let rt = tokio::runtime::Runtime::new()?;

        rt.block_on(async {
            let client = Surreal::new::<Ws>("localhost:8000").await?;

            let db = Database {
                client,
                command_receiver,
                response_sender,
            };

            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

            db.use_ns("test").use_db("test").await?;

            db.query(
                "DEFINE TABLE person SCHEMALESS
                    PERMISSIONS FOR
                        CREATE, SELECT WHERE $auth,
                        FOR UPDATE, DELETE WHERE created_by = $auth;
                DEFINE FIELD name ON TABLE person TYPE string;
                DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;

                DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
                DEFINE ACCESS account ON DATABASE TYPE RECORD
                SIGNUP ( CREATE user SET name = $name, pass = \
                        crypto::argon2::generate($pass) )
                SIGNIN ( SELECT * FROM user WHERE name = $name AND \
                        crypto::argon2::compare(pass, $pass) )
                DURATION FOR TOKEN 15m, FOR SESSION 12h
                ;",
            )
            .await?;

            loop {
                if let Ok(command) = db.command_receiver.try_recv() {
                    match db.handle_command(command).await {
                        | Ok(s) => db.response_sender.send(s)?,
                        | Err(e) => db.response_sender.send(e.to_string())?,
                    }
                }
            }
        })
    });

    let app = SurrealDbApp {
        current_tab: AppTab::People,

        // Person Management
        person_name: String::new(),
        person_id_to_delete: String::new(),
        people_list: String::new(),

        // Authentication
        auth_username: String::new(),
        auth_password: String::new(),
        current_user: String::new(),

        // Query
        raw_query: String::new(),
        query_result: String::new(),

        // Session
        session_info: String::new(),

        // Messages and Status
        messages: Vec::new(),
        is_loading: false,
        connection_status: "Connected to localhost:8000".to_string(),

        // Communication
        command_sender,
        response_receiver,
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let _ = eframe::run_native("SurrealDB App", native_options, Box::new(|_cc| Ok(Box::new(app))));
    Ok(())
}
