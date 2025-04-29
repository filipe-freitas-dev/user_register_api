use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{database::user_models::User, repositories::user_repo::UserRepository};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users, get_user]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    id: Uuid,
    name: String,
    email: String,
}

impl UserResponse {
    pub fn from_user(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

#[get("/users")]
pub async fn get_users(service: &State<UserRepository>) -> Result<Json<Vec<UserResponse>>, Status> {
    let users = service.get_users();
    match users {
        Ok(users) => Ok(Json(
            users.into_iter().map(UserResponse::from_user).collect(),
        )),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<id>")]
pub async fn get_user(
    id: &str,
    service: &State<UserRepository>,
) -> Result<Json<UserResponse>, Status> {
    let _id = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    match service.get_user(_id) {
        Ok(user) => Ok(Json(UserResponse::from_user(user))),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}
