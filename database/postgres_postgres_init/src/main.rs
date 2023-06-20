use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

struct Author {
    _id: i64,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgrespassword@localhost:5432/",
        NoTls,
    )?;

    client.batch_execute(
        "
    CREATE TABLE IF NOT EXISTS author (
        id      SERIAL PRIMARY KEY,
        name    VARCHAR NOT NULL,
        country VARCHAR NOT NULL
    )
    ",
    )?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Adam"), "France");
    authors.insert(String::from("Ben"), "German");
    authors.insert(String::from("Chris"), "UK");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author  (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    for record in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: record.get(0),
            name: record.get(1),
            country: record.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
