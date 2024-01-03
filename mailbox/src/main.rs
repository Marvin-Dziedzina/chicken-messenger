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

const USERS_DB: &str = "users.json";
const MESSAGES_DB: &str = "messages.json";

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

#[get("/is_user_hash_existent?<user_hash>")]
async fn is_user_hash_existent(user_hash: &str) -> String {
    let users = database::get_save_data(USERS_DB);
    let x = &users[user_hash];

    let content = match x {
        serde_json::Value::Null => false,
        _ => true,
    };

    serde_json::to_string(&responses::Response::new(false, "", content))
        .expect("Could not convert struct to json string!")
}

/// Returns empty strings if not found
#[get("/name_to_user_hash?<user_name>")]
async fn name_to_user_hash(user_name: String) -> String {
    let users = database::get_save_data(USERS_DB);
    let mut user_hash = String::new();
    for user in users.as_object().expect("Could not iter over user db!") {
        if user.1["user_name"] == user_name {
            user_hash = user.1["user_hash"].to_string();
            break;
        }
    }

    if user_hash.is_empty() {
        return serde_json::to_string(&responses::Response::new(
            false,
            "",
            responses::NameToUserHashContent::new("".to_string(), "".to_string()),
        ))
        .expect("Could not convert struct to json string!");
    }

    serde_json::to_string(&responses::Response::new(
        false,
        "",
        responses::NameToUserHashContent::new(user_name, user_hash),
    ))
    .expect("Could not convert struct to json string!")
}

#[launch]
fn rocket_start() -> _ {
    rocket::build().mount(
        "/",
        routes![index, is_user_hash_existent, name_to_user_hash],
    )
}
