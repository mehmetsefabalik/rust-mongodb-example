use bson::{doc, ordered::OrderedDocument};
use mongodb::{error::Error, results::InsertOneResult, Collection};

#[derive(Clone)]
pub struct UserService {
  collection: Collection,
}

impl UserService {
  pub fn new(collection: Collection) -> UserService {
    UserService { collection }
  }

  pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
    self.collection.insert_one(doc! {"name": name}, None)
  }

  pub fn get(&self) -> Result<Option<OrderedDocument>, Error> {
    self.collection.find_one(doc! {}, None)
  }
}
