use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySqlPool}; // SQLx types: Error for error handling, FromRow for deserialization, and MySqlPool for the database pool

// Struct to capture the data needed to create a new Todo
#[derive(Serialize, Deserialize)]
struct CreateNewTodos {
    title: String,
    description: Option<String>, // Optional field
}

// Main Todo struct representing a single entry in the "todos" table
// Fields should match the columns in the database
#[derive(Serialize, Deserialize, FromRow)] // FromRow allows SQLx to map query results to struct fields
pub struct Todo {
    id: i32, // Unique ID for each todo entry
    title: String,
    description: Option<String>,
    status: bool,
}

// Struct to send error messages in JSON format if a database error occurs
#[derive(Serialize, Deserialize)]
struct TypeDbError {
    error: String, // Stores the error message to return in the response
}

// Endpoint to create a new todo entry
#[post("/todo/create")]
pub async fn create_new_todo(db: Data<MySqlPool>, body: Json<CreateNewTodos>) -> impl Responder {
    println!("1");

    // Execute SQL query to insert a new todo into the database
    // "?" placeholders bind values safely to avoid SQL injection
    let response = sqlx::query("INSERT INTO todos(title, description) VALUES(?,?)")
        .bind(&body.title) // Bind title field from request body
        .bind(&body.description) // Bind description field from request body
        .execute(&**db) // Execute query using the database connection
        .await;
    println!("123");

    // Match on the result to handle success or error cases
    match response {
        Ok(result) => HttpResponse::Created().json(Todo {
            id: result.last_insert_id() as i32,    // Get ID of the new record
            title: body.title.clone(),             // Clone title from request body
            description: body.description.clone(), // Clone description from request body
            status: false, // Initialize status as false (or you could use body.status if it's provided)
        }),
        Err(e) => HttpResponse::InternalServerError().json(TypeDbError {
            error: e.to_string(), // Convert the error to a string and return it in JSON format
        }),
    }
}

// Endpoint to fetch all todos from the database
#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    // Define the SQL query to select all columns from the todos table.
    let response: Result<Vec<Todo>, Error> =
        sqlx::query_as("SELECT id, title, description, status FROM todos")
            .fetch_all(&**db) // Fetch all rows that match the query as a Vec<Todo>.
            .await; // Wait for the query to complete asynchronously.

    match response {
        Ok(todos) => HttpResponse::Ok().json(todos), // If successful, return todos in JSON format.
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().json(TypeDbError {
                // If there's an error, respond with a 404 and the error message.
                error: e.to_string(),
            })
        }
    }
}