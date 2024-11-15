use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    user_id: i32,
    title: String,
    description: Option<String>,
    status: bool,
    // created_at: chrono::NaiveDateTime,
}