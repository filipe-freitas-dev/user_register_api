use repositories::user_repo::UserRepository;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .max_age(Some(3600))
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Delete,
                Method::Put,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    let user_repo = UserRepository::new(pool);

    rocket::build()
        .manage(user_repo)
        .mount("/", services::user_routes::routes())
        .mount("/", services::auth_routes::routes())
        .attach(cors.to_cors().unwrap())
}
