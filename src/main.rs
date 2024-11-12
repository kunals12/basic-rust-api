use actix_web::{
    get,               // Macro for defining HTTP GET request handlers
    web::{self, Path}, // `web` for handling web resources, `Path` for capturing URL path parameters
    App,
    HttpResponse,
    HttpServer,
    Responder, // Core Actix components for setting up the web server and responses
};

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
async fn hello_user(params: Path<(String, u32)>) -> impl Responder {
    // Format the dynamic parameters into a greeting message
    let response = format!("Hello {} {}", params.0, params.1);
    // Return the response as the HTTP body
    response
}

#[actix_web::main] // Macro to mark the main async function for running Actix's async runtime
async fn main() -> std::io::Result<()> {
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

    // Await the server to keep it running until it's stopped
    server.await
}
