#[macro_use]
extern crate rocket;
use rocket::{
    figment::providers::Serialized,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde_json;

mod responses;
mod saves;

#[get("/")]
async fn index() -> String {
    let response = serde_json::to_string(&responses::Timestamp::now())
        .expect("Could not convert struct to json!");

    response
}

#[launch]
fn rocket_start() -> _ {
    rocket::build().mount("/", routes![index])
}
