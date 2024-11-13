use actix_web::{web::Data, App, HttpServer};
mod routes;
use routes::*;
mod database;
use database::*;

#[tokio::main] // Macro to mark the main async function for running Actix's async runtime
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    let database = database_connection()
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");
    // Initialize the server and define the routes and services
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            // Register the `hello` service route with the server
            .service(hello)
            .service(get_all_todos)
            // Register the `hello_user` service route, which includes path parameters
            .service(hello_user)
            .service(create_user)
            .service(create_new_todo)
    })
    // Bind the server to a local address and port (127.0.0.1:8080)
    .bind(("127.0.0.1", 8080))?
    // Start the server asynchronously
    .run();
    println!("Server is running on port {}", port);
    // Await the server to keep it running until it's stopped
    server.await
}
