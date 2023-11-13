#[macro_use]
extern crate rocket;

struct Request {
    user_name: String,
    pw_hash: String,
}

#[get("/")]
fn home() -> &'static str {
    "Hello World"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![home])
}
