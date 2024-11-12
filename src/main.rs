use actix_web::{App, HttpServer};
mod routes;
use routes::*;

#[actix_web::main] // Macro to mark the main async function for running Actix's async runtime
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    // Initialize the server and define the routes and services
    let server = HttpServer::new(|| {
        App::new()
            // Register the `hello` service route with the server
            .service(hello)
            // Register the `hello_user` service route, which includes path parameters
            .service(hello_user)
    })
    // Bind the server to a local address and port (127.0.0.1:8080)
    .bind(("127.0.0.1", 8080))?
    // Start the server asynchronously
    .run();
    println!("Server is running on port {}", port);
    // Await the server to keep it running until it's stopped
    server.await
}
