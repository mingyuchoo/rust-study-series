// #[post("/path", data= "<body>")]
// #[get("/path?<page>&<limit>")]
// #[get("/path/<id>")]
// #[put("/path/<id>", data = "<body>")]
// #[patch("/path/<id>", data = "<body>")]
// #[delete("/path/<id>")]

use serde::Serialize;

pub mod health;
pub mod home;
pub mod posts;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SingleDataResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct MultiDataResponse<T> {
    pub status: String,
    pub data: Vec<T>,
}
