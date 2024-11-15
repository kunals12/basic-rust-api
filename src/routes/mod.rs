pub mod user;
use serde::{Deserialize, Serialize};
pub use user::{create_user};
pub mod home;
pub use home::hello;
pub mod todos;
pub use todos::*;

pub fn logging(path: &str) {
    println!("{}", path)
}

// Struct to send error messages in JSON format if a database error occurs
#[derive(Serialize, Deserialize)]
struct TypeDbError {
    error: String, // Stores the error message to return in the response
}