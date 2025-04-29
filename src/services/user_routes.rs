use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::AuthToken,
    database::user_models::{NewUser, PartialUpdateUser, User},
    repositories::user_repo::UserRepository,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_users,
        get_user,
        create_user,
        delete_user,
        update_user,
        change_password
    ]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdResponse {
    id: Uuid,
}

impl UserIdResponse {
    pub fn from_user(user: User) -> UserIdResponse {
        UserIdResponse { id: user.id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordProps {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[get("/users")]
pub async fn get_users(
    service: &State<UserRepository>,
    _auth: AuthToken,
) -> Result<Json<Vec<UserResponse>>, Status> {
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
    _auth: AuthToken,
) -> Result<Json<UserResponse>, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    match service.get_user(uuid) {
        Ok(user) => Ok(Json(UserResponse::from_user(user))),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(
    user: Json<NewUser>,
    service: &State<UserRepository>,
) -> Result<Json<UserIdResponse>, Status> {
    let user = user.into_inner();
    let hashed_password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).unwrap();
    let user = NewUser {
        name: user.name,
        email: user.email,
        password: hashed_password,
    };

    if user.password.is_empty() {
        return Err(Status::BadRequest);
    }
    if user.password.len() < 8 {
        return Err(Status::BadRequest);
    }

    match service.create_user(user) {
        Ok(user_created) => {
            let user_created = UserIdResponse::from_user(user_created);
            Ok(Json(user_created))
        }
        Err(diesel::result::Error::DatabaseError(_, _)) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/users/<id>")]
pub async fn delete_user(
    id: &str,
    service: &State<UserRepository>,
    auth: AuthToken,
) -> Result<Status, Status> {
    let user_id = auth.extract_user_id().unwrap();
    if user_id != id {
        return Err(Status::Unauthorized);
    }
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    match service.delete_user(uuid) {
        Ok(_) => Ok(Status::NoContent),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub async fn update_user(
    id: &str,
    user: Json<PartialUpdateUser>,
    service: &State<UserRepository>,
    auth: AuthToken,
) -> Result<Status, Status> {
    let mut user = user.into_inner();
    if user.email.is_none() && user.name.is_none() {
        return Err(Status::BadRequest);
    }

    if user.password.is_some() {
        return Err(Status::BadRequest);
    }

    user.password = None;

    let user_id = auth.extract_user_id().unwrap();
    if user_id != id {
        return Err(Status::Unauthorized);
    }
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    match service.update_user(uuid, user) {
        Ok(_) => Ok(Status::NoContent),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/change-password", format = "json", data = "<user>")]
pub async fn change_password(
    service: &State<UserRepository>,
    user: Json<ChangePasswordProps>,
    auth: AuthToken,
) -> Result<Status, Status> {
    let user_id = auth.extract_user_id().unwrap();
    let uuid = Uuid::parse_str(user_id).map_err(|_| Status::BadRequest)?;
    let db_user = service.get_user(uuid).map_err(|_| Status::BadRequest)?;

    let user = user.into_inner();
    if user.old_password.is_empty() {
        return Err(Status::BadRequest);
    }

    let is_valid = bcrypt::verify(&user.old_password, &db_user.password).unwrap();
    if !is_valid {
        return Err(Status::Unauthorized);
    }

    if user.new_password.is_empty() {
        return Err(Status::BadRequest);
    }
    if user.new_password.len() < 8 {
        return Err(Status::BadRequest);
    }

    if user.new_password != user.confirm_password {
        return Err(Status::BadRequest);
    }

    match service.change_password(uuid, &user.new_password) {
        Ok(_) => Ok(Status::NoContent),
        Err(diesel::result::Error::NotFound) => Err(Status::BadRequest),
        Err(_) => Err(Status::InternalServerError),
    }
}
