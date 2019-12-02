use actix_web::{http, web, Error, HttpResponse};
use futures::future::Future;
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
  name: String,
}

pub fn index(
  user: web::Query<User>,
  pool: web::Data<Pool<MongodbConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || crate::service::index(&user.name, pool)).then(|_result| match _result {
    Ok(_) => HttpResponse::Ok().body("Success"),
    Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  })
}
