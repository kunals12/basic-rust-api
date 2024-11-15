use actix_web::{
    delete, get, post, web::{Data, Json, Path}, HttpResponse, Responder
};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySqlPool};
use crate::routes::TypeDbError; // SQLx types: Error for error handling, FromRow for deserialization, and MySqlPool for the database pool

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

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoTitle {
    id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<bool>
}


// Endpoint to create a new todo entry
#[post("/todo/create")]
pub async fn create_new_todo(db: Data<MySqlPool>, body: Json<CreateNewTodos>) -> impl Responder {
    // Execute SQL query to insert a new todo into the database
    // "?" placeholders bind values safely to avoid SQL injection
    let response = sqlx::query("INSERT INTO todos(title, description) VALUES(?,?)")
        .bind(&body.title) // Bind title field from request body
        .bind(&body.description) // Bind description field from request body
        .execute(&**db) // Execute query using the database connection
        .await;

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

// Endpoint to update todo
#[post("todo/update")]
pub async fn update_todo(db: Data<MySqlPool>, body: Json<UpdateTodoTitle>) -> impl Responder {

    let mut fields_to_update = Vec::new();

    // Add fields to update query conditionally based on their presence
    if let Some(title) = &body.title {
        fields_to_update.push(format!("title = '{}'", title));
    }

    if let Some(description) = &body.description {
        fields_to_update.push(format!("description = '{}'", description));
    }

    if let Some(status) = body.status {
        fields_to_update.push(format!("status = {}", status));
    }

    if fields_to_update.is_empty() {
        return HttpResponse::BadRequest().json(TypeDbError {
            error: "No fields to update".to_string(),
        });
    }

    // Construct the SQL query
    let query = format!("UPDATE todos SET {} WHERE id = ?", fields_to_update.join(", "));
    
    // Execute the query with the ID parameter
    let response = sqlx::query(&query)
        .bind(body.id)
        .execute(&**db)
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().json(UpdateTodoTitle {
            id: body.id,
            title: body.title.clone(),
            description: body.description.clone(),
            status: body.status
        }),
        Err(e) => HttpResponse::InternalServerError().json(TypeDbError {
            error: e.to_string()
        })
    }
}

#[delete("todo/delete/{id}")]
pub async fn delete_todo(db: Data<MySqlPool>, params:Path<i32>) -> impl Responder {
    let response = sqlx::query("DELETE FROM todos WHERE id = ?").bind(params.into_inner()).execute(&**db).await;

    // Handle the result of the query
    match response {
        Ok(result) => {
            // Check if any row was affected (meaning a record was deleted)
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json("Todo successfully deleted")
            } else {
                HttpResponse::NotFound().json("Todo not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e)),
    }
}