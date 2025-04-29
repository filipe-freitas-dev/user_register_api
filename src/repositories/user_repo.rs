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

    pub fn get_users(&self) -> Result<Vec<User>, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let _users = users.load::<User>(&mut conn)?;
        Ok(_users)
    }

    pub fn get_user(&self, _id: Uuid) -> Result<User, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let user = users.find(_id).first(&mut conn)?;
        Ok(user)
    }
}
