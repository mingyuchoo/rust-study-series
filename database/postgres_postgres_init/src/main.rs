use dotenv;
use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let url: String = dotenv::var("DATABASE_URL").unwrap();
    let mut client: Client = Client::connect(url.as_str(), NoTls)?;

    client.batch_execute(
        "
    CREATE TABLE IF NOT EXISTS author (
        id      SERIAL PRIMARY KEY,
        name    VARCHAR NOT NULL,
        country VARCHAR NOT NULL
    )
    ",
    )?;

    let mut authors: HashMap<String, &str> = HashMap::new();
    authors.insert(String::from("Adam"), "France");
    authors.insert(String::from("Ben"), "German");
    authors.insert(String::from("Chris"), "UK");

    for (key, value) in &authors {
        let author: Author = Author {
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
        let author: Author = Author {
            _id: record.get(0),
            name: record.get(1),
            country: record.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
