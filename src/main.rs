use actix_web::{
    get,
    http::StatusCode,
    web::{self, Json, Path},
    App,
    HttpResponse,
    HttpServer,
    Responder, // Core Actix components for setting up the web server and responses
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
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

// Define a basic route that responds to GET requests at "/hello"
#[get("/hello")]
async fn hello() -> impl Responder {
    // Simple response with a static string
    let response = "Hello World";
    // Return the response as the HTTP body
    response
}

// Define a route that accepts dynamic path parameters: `/hello/{firstname}/{lastname}`
#[get("hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    // Format the dynamic parameters into a greeting message
    let response = User::new(params.0.clone(), params.1.clone(), 18);
    // Return the response as the HTTP body
    (Json(response), StatusCode::OK)
}

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
