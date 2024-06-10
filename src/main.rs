use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User{
    name: String, 
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

//define out endpoint.
#[actix_web::get("/users/{id}")]
async fn get_user(
    user_id: web::Path<u32>, 
    db: web::Data<UserDb>) -> Result<impl Responder> {
    format!("Hello {user_id}!")
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    id: u32, 
    name: String,
}

#[actix_web::post("/users")]
async fn create_user(
    user_data: web::Json<User>, 
    db: web::Data<UserDb>
) -> impl Responder{
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse{
        id: new_id,
        name,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(create_user)
    })
        .bind(("127.0.01", port))?
        .workers(2)
        .run()
        .await
}
