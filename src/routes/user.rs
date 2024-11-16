use crate::routes::{logging, Message, TypeDbError};
use actix_web::{
    get, post, web::{Data, Json, Path}, HttpResponse, Responder // Core Actix components for setting up the web server and responses
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use crate::routes::todos::Todo;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct CreateUserResponse {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserWithTodosResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub todos: Vec<Todo>, // Array of todos for this user
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

#[get("/user/{id}")]
pub async fn get_user_by_id(db: Data<MySqlPool>, params: Path<i32>) -> impl Responder {
    // Fetch the user by ID
    let user = sqlx::query_as::<_, CreateUserResponse>("SELECT id, username, email, created_at FROM users WHERE id = ?")
        .bind(params.into_inner()) // Use params.0 to access the ID
        .fetch_one(&**db) // Execute the query
        .await;

    // Handle the result
    match user {
        Ok(user) => {
            let todos = sqlx::query_as(
                "SELECT id, title, description, status FROM todos WHERE user_id = ?"
            ).bind(user.id).fetch_all(&**db).await;

            match todos {
                Ok(todos) => HttpResponse::Ok().json(UserWithTodosResponse {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    todos, // Attach todos to the response
                }),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch todos"),
            }
            // HttpResponse::Ok().json(user)
        }, // Return user data as JSON
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json("User Not Found"), // Handle no row found
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()), // Handle other DB errors
    }
}


