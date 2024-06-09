use actix_web::{App, HttpServer};

async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    HttpServer::new(|| App::new())
        .bind(("127.0.01", port))?
        .run()
        .await
}
