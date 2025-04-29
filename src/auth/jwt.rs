use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self { secret }
    }

    pub fn create_token(&self, user_id: String) -> Result<String, Error> {
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("invalid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: exp as usize,
        };

        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());
        encode(&Header::default(), &claims, &encoding_key)
    }
}
