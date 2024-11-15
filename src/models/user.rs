use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    // created_at: 
}