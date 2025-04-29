use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth, database::user_models::User, repositories::user_repo::UserRepository};

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    id: Uuid,
    token: String,
}

impl LoginResponse {
    pub fn from_user(user: User, token: String) -> LoginResponse {
        LoginResponse { id: user.id, token }
    }
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<LoginRequest>,
    service: &State<UserRepository>,
) -> Result<Json<LoginResponse>, Status> {
    let user = user.into_inner();
    let user_from_db = service.get_by_email(&user.email).unwrap();
    if !bcrypt::verify(user.password, &user_from_db.password).unwrap() {
        return Err(Status::Unauthorized);
    }

    let jwt_service = auth::jwt::JwtService::new();
    let token = jwt_service
        .create_token(user_from_db.id.to_string())
        .unwrap();
    Ok(Json(LoginResponse::from_user(user_from_db, token)))
}
