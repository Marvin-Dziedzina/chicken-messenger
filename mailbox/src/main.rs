#[macro_use]
extern crate rocket;
use rocket::{
    figment::providers::Serialized,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde_json;

mod database;
mod responses;

#[get("/")]
async fn index() -> String {
    let response = serde_json::to_string(&responses::Response::new(
        false,
        "",
        responses::NoContent::new(),
    ))
    .expect("Could not convert struct to json!");

    response
}

#[get("/is_user_hash_existent?<user>")]
async fn is_user_hash_existent(user: &str) -> String {
    let users = database::get_save_data("users.json");
    let x = &users[user];

    let content = match x {
        serde_json::Value::Null => false,
        _ => true,
    };

    serde_json::to_string(&responses::Response::new(false, "", content))
        .expect("Could not convert string to json!")
}

#[launch]
fn rocket_start() -> _ {
    rocket::build().mount("/", routes![index, is_user_hash_existent])
}
