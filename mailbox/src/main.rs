use actix_web::{delete, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::{self, Value};
use sqlite;

mod config;

// User manager api points
#[post("/create_user")]
async fn create_user(req_body: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
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
    HttpServer::new(|| App::new().service(get_salt).service(create_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
