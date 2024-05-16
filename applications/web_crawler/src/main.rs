use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://jsonplaceholder.typicode.com";
    let client = Client::new();
    let res = client.get(url).send()?.text();
    let document = Document::from(res?.as_str());

    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            println!("{}", href);
        }
    }
    Ok(())
}
