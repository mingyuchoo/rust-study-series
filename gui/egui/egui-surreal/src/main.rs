use anyhow::Error;
use egui::RichText;
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
                Ok(format!("Back to root!"))
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
    input: String,
    results: String,
    command_sender: Sender<Command>,
    response_receiver: Receiver<String>,
}

impl SurrealDbApp {
    fn send(&mut self, command: Command) {
        if let Err(e) = self.command_sender.send(command) {
            self.results = e.to_string()
        }
    }
}

impl eframe::App for SurrealDbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left").show(ctx, |ui| {
            if let Ok(response) = self.response_receiver.try_recv() {
                self.results = response;
            }
            if ui.button("Create person").clicked() {
                self.send(Command::CreatePerson(self.input.clone()))
            };
            if ui.button("Delete person").clicked() {
                self.send(Command::DeletePerson(self.input.clone()))
            }
            if ui.button("List people").clicked() {
                self.send(Command::ListPeople)
            }
            if ui.button("Session data").clicked() {
                self.send(Command::Session)
            }
            if ui.button("New user").clicked() {
                self.send(Command::SignUp)
            }
            if ui.button("Sign in as record user").clicked() {
                self.send(Command::SignIn(self.input.clone()));
            }
            if ui.button("Sign in as root").clicked() {
                self.send(Command::SignInRoot)
            }
            if ui.button("Raw query").clicked() {
                self.send(Command::RawQuery(self.input.clone()))
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Input:").heading());
            ui.text_edit_multiline(&mut self.input);
        });
        egui::SidePanel::right("right").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new("Results:").heading());
                ui.text_edit_multiline(&mut self.results);
            });
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
        input: String::new(),
        results: String::new(),
        command_sender,
        response_receiver,
    };

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("SurrealDB App", native_options, Box::new(|_cc| Ok(Box::new(app))));
    Ok(())
}
