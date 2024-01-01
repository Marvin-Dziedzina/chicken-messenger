#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket_start() -> _ {
    rocket::build().mount("/", routes![index])
}
