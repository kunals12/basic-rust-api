use crate::routes::logging;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Json, Path},
    Responder, // Core Actix components for setting up the web server and responses
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    id: u32,
    user: User,
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
pub async fn hello_user(params: Path<(String, String, u8)>) -> impl Responder {
    let route = format!("GET: /{}/{}", params.0.clone(), params.1.clone());
    logging(&route);
    // Format the dynamic parameters into a greeting message
    let response = User::new(params.0.clone(), params.1.clone(), params.2);
    // Return the response as the HTTP body
    (Json(response), StatusCode::OK)
}

#[post("/user/create")]
pub async fn create_user(user: Json<User>) -> impl Responder {
    logging("GET /user/create");
    (
        Json(CreateUserResponse {
            id: 1,
            user: user.0,
        }),
        StatusCode::CREATED,
    )
}
