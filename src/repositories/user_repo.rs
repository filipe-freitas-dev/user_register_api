use crate::database::{DbPool, user_models::User};
use diesel::prelude::*;
use uuid::Uuid;

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn get_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let _users = users.load::<User>(&mut conn)?;
        Ok(_users)
    }
}
