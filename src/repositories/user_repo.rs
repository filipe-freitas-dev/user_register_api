use crate::database::{
    DbPool,
    user_models::{NewUser, User},
};
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

    pub fn create_user(&self, user: NewUser) -> Result<User, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let user = diesel::insert_into(users)
            .values(&user)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn get_by_email(&self, _email: &str) -> Result<User, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let user = users.filter(email.eq(_email)).first(&mut conn)?;
        Ok(user)
    }
}
