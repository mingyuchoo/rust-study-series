use rocket::local::blocking::Client;
use hello_rocket::routes;

fn get_client() -> Client {
    Client::tracked(routes::build()).expect("valid rocket instance")
}

#[cfg(test)]
mod posts {
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_blocking_get_posts() {
        let client: Client = super::get_client();
        let response = client.get("/posts").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}