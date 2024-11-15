use crate::routes::{logging, Message, TypeDbError};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
    Responder, // Core Actix components for setting up the web server and responses
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    id: i32,
    username: String,
    email: String,
}

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Error Hashing Password")
}

fn verify_password(password: &str, hash_password: &str) -> bool {
    verify(password, hash_password).unwrap_or(false)
}

async fn check_user_exists(db: Data<MySqlPool>, email: &str) -> bool {
    let is_user_exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
        .bind(email)
        .fetch_one(&**db)
        .await
        .unwrap_or(false);

    is_user_exists
}

#[post("/user/create")]
pub async fn create_user(db: Data<MySqlPool>, user: Json<CreateUserRequest>) -> impl Responder {
    logging("POST /user/create");

    let is_user_exists = check_user_exists(db.clone(), &user.email).await;

    // Check if the user already exists and return a response early
    if is_user_exists {
        return HttpResponse::Conflict().json(Message {
            msg: "User Already Exists".to_string()
        });
    }

    // If user doesn't exist, proceed with hashing password and inserting new user
    let hash_password: String = hash_password(&user.password);

    let result = sqlx::query("INSERT INTO users (username, email, password) VALUES (?,?,?)")
        .bind(user.username.clone())
        .bind(user.email.clone())
        .bind(hash_password)
        .execute(&**db)
        .await;

    match result {
        Ok(data) => HttpResponse::Created().json(CreateUserResponse {
            id: data.last_insert_id() as i32,
            username: user.username.clone(),
            email: user.email.clone(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(TypeDbError {
            error: err.to_string(),
        }),
    }
}
