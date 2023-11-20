use actix_web::{delete, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use json;
use serde_json::{self, Value};
use sqlite;

mod config;
mod query_params;
mod responses;
mod sqlite_hdlr;

use sqlite_hdlr::SqLiteHDLR;

const DATABASE_PATH: &str = "database.sqlite";

// User manager api points
#[post("/create_user")]
async fn create_user(
    req_body: HttpRequest,
    account_data: web::Query<query_params::PwParam>,
) -> impl Responder {
    fn add_user(user_name: String, password: String, salt: String) {}

    let config_data = config::read_config().await;

    if config_data.auth_password.is_empty() {
        //add_user(user_name, password, salt);
        let sqlite_instance = SqLiteHDLR::new(DATABASE_PATH);
        sqlite_instance.execute("")
    } else if config_data.auth_pw_hash != account_data.auth_password_hash {
        let response =
            responses::DefaultResponse::new(false, "", false, responses::AuthResponse::new("", ""));
        let response = serde_json::to_string(&response);
        let response = responses::handle_json_error(response);

        return HttpResponse::Ok().body(response);
    }

    // success

    let response =
        responses::DefaultResponse::new(false, "", true, responses::AuthResponse::new("", ""));
    let response = serde_json::to_string(&response);
    let response = responses::handle_json_error(response);

    HttpResponse::Ok().body(response)
}

#[delete("/delete_user")]
async fn delete_user(req_body: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

// Login
#[get("/get_salt")]
async fn get_salt(req_body: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sqlite_hdlr::SqLiteHDLR::new(DATABASE_PATH);

    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(delete_user)
            .service(get_salt)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
