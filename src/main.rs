use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};
use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User{
    name: String, 
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

//define out endpoint.
#[actix_web::get("/greet/{id}")]
async fn greet(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello {user_id}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data).service(greet)
    })
        .bind(("127.0.01", port))?
        .workers(2)
        .run()
        .await
}
