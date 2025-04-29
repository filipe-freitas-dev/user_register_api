use super::schema::admins;
use super::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = admins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = admins)]
pub struct NewAdmin {
    pub user_id: Uuid,
}

#[derive(Debug, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct PartialUpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
