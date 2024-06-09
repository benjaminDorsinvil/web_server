use actix_web::{App, HttpServer, Responder};

//define out endpoint.
#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    HttpServer::new(|| App::new().service())
        .bind(("127.0.01", port))?
        .workers(2)
        .run()
        .await
}
