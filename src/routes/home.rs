use crate::routes::logging;
use actix_web::{get, Responder};

// Define a basic route that responds to GET requests at "/hello"
#[get("/")]
pub async fn hello() -> impl Responder {
    logging("GET: /");
    // Simple response with a static string
    let response = "Welcome to basic web server, powered by Kunal Sonwane";
    // Return the response as the HTTP body
    response
}
