use repositories::user_repo::UserRepository;

#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod repositories;
mod services;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let pool = database::establish_connection_pool();

    let user_repo = UserRepository::new(pool);
    rocket::build()
        .manage(user_repo)
        .mount("/", services::user_routes::routes())
}
