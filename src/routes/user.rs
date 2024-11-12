use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    Responder, // Core Actix components for setting up the web server and responses
};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

impl User {
    pub fn new(first_name: String, last_name: String, age: u8) -> Self {
        User {
            first_name,
            last_name,
            age,
        }
    }
}

// Define a route that accepts dynamic path parameters: `/hello/{firstname}/{lastname}`
#[get("{firstname}/{lastname}")]
pub async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    // Format the dynamic parameters into a greeting message
    let response = User::new(params.0.clone(), params.1.clone(), 18);
    // Return the response as the HTTP body
    (Json(response), StatusCode::OK)
}
