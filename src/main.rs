use actix_web::{web, App, HttpServer};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

mod controller;
mod service;

fn main() {
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            .with_db("mydb")
            .build(),
    );

    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/hello", web::get().to_async(controller::index))
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .unwrap();
}
