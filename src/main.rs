use actix_web::{
    get,
    web::{self, Path},
    App, HttpResponse, HttpServer, Responder,
};

#[get("/hello")]
async fn hello() -> impl Responder {
    let response = "Hello World";
    response
}

#[get("hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, u32)>) -> impl Responder {
    let response = format!("Hello {} {}", params.0, params.1);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(hello).service(hello_user))
        .bind(("127.0.0.1", 8080))?
        .run();
    server.await
}
