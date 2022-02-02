use mongodb::{error::Error, results::InsertOneResult, sync::Collection};

#[derive(Clone)]
pub struct UserService {
    collection: mongodb::sync::Collection<bson::Document>,
}

impl UserService {
    pub fn new(collection: Collection<bson::Document>) -> UserService {
        UserService { collection }
    }

    pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(bson::doc! {"name": name}, None)
    }

    pub fn get(&self) -> Result<Option<bson::Document>, Error> {
        self.collection.find_one(bson::doc! {}, None)
    }
}
