use crate::database::{
    DbPool,
    user_models::{NewUser, PartialUpdateUser, User},
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

    pub fn get_user(&self, uuid: Uuid) -> Result<User, diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let user = users.find(uuid).first(&mut conn)?;
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

    pub fn delete_user(&self, uuid: Uuid) -> Result<(), diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        let deleted_users = diesel::delete(users.find(uuid)).execute(&mut conn)?;

        if deleted_users == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(())
    }

    pub fn update_user(
        &self,
        uuid: Uuid,
        user: PartialUpdateUser,
    ) -> Result<(), diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        diesel::update(users.find(uuid))
            .set(&user)
            .get_result::<User>(&mut conn)?;
        Ok(())
    }

    pub fn change_password(
        &self,
        uuid: Uuid,
        new_password: &str,
    ) -> Result<(), diesel::result::Error> {
        use crate::database::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Failed to get connection");
        diesel::update(users.find(uuid))
            .set(password.eq(new_password))
            .get_result::<User>(&mut conn)?;
        Ok(())
    }
}
