use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode, errors::Error};
use jwt::Claims;
use rocket::{
    Request,
    http::Status,
    request::{self, FromRequest, Outcome},
};

pub mod jwt;

pub struct AuthToken {
    claims: Claims,
}

impl AuthToken {
    pub fn new(claims: Claims) -> Self {
        Self { claims }
    }

    pub fn extract_user_id(&self) -> Result<&String, Error> {
        Ok(&self.claims.sub)
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(auth_header) => auth_header,
            None => return Outcome::Error((Status::Unauthorized, Self::Error::MissingToken)),
        };

        let token = token
            .strip_prefix("Bearer ")
            .ok_or(Self::Error::InvalidToken)
            .expect("Invalid token format");

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(token_data) => Outcome::Success(AuthToken::new(token_data.claims)),
            Err(_) => Outcome::Error((Status::Unauthorized, Self::Error::InvalidToken)),
        }
    }
}
