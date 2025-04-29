use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    id: Uuid,
    name: String,
    email: String,
}

#[get("/users")]
pub async fn get_users() -> Result<Json<UserResponse>, Status> {
    let users;
}
