use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use service::UserService;

mod controller;
mod service;

pub struct ServiceContainer {
  user: UserService,
}

impl ServiceContainer {
  pub fn new(user: UserService) -> Self {
    ServiceContainer { user }
  }
}

pub struct AppState {
  service_container: ServiceContainer,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
  let client = Client::with_options(client_options).unwrap();
  let db = client.database("mydb");

  let user_collection = db.collection("users");

  HttpServer::new(move || {
    let service_container = ServiceContainer::new(UserService::new(user_collection.clone()));

    App::new()
      .data(AppState { service_container })
      .route("/hello", web::get().to(controller::index))
      .route("/get", web::get().to(controller::get))
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
