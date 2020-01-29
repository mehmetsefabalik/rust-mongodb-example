use actix_web::web;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

pub fn index(
  name: &str,
  pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<mongodb::coll::results::InsertOneResult, mongodb::error::Error> {
  pool
    .get()
    .expect("can not get pool")
    .collection("users")
    .insert_one(doc! {"name" => name}, None)
}