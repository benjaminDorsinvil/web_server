// import libraries
use std::{
    sync::{Arc, Mutex},
    collections::HashMap
};
use actix_web::{
    web, 
    App, 
    HttpServer, 
    Responder, 
    HttpResponse, 
    Error, 
    error::ErrorNotFound
};
use serde::{
    Deserialize, 
    Serialize
};


// User struct 
#[derive(Serialize, Deserialize)]
struct User{
    name: String, 
    age: u32,
}

//  Create a local hashmap as a temporary local database
type UserDb = Arc<Mutex<HashMap<u32, User>>>;

// creating the get request by matching user id to hashmap database 
#[actix_web::get("/users/{id}")]
async fn get_user(
    user_id: web::Path<u32>, 
    db: web::Data<UserDb>) -> Result<impl Responder, Error> {
        let user_id = user_id.into_inner();
        let db = db.lock().unwrap();
        match db.get(&user_id) {
            Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
            None => Err(ErrorNotFound("User not found")),
        }
        
}

// Creating user response struct
#[derive(Serialize)]
struct CreateUserResponse {
    id: u32, 
    name: String,
    age: u32
}

// endpoint to create new user
#[actix_web::post("/users")]
async fn create_user(
    user_data: web::Json<User>, 
    db: web::Data<UserDb>
) -> impl Responder{
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    let age = user_data.age.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse{
        id: new_id,
        name,
        age,
    })
}

// main function creates address and port and runs the app
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(get_user)
            .service(create_user)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
