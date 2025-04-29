#[macro_use]
extern crate rocket;

mod database;

#[get("/")]
async fn index() -> &'static str {
    "hello world"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
