use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

#[derive(Deserialize)]
pub struct UserForm {
    pub name: String,
    pub age: u32,
}