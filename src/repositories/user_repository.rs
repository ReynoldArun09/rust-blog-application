use crate::{error::AppError, models::user::User};
use mongodb::{Collection, bson::doc};

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }

    pub async fn find_by_email(&self, email: &string) -> Result<Option<User>, AppError> {
        Ok(self.collection.find_one(doc! {"email": email}).await?)
    }

    pub async fn create(&self, user: &User) -> Result<mongodb::result::InsertOneResult, AppError> {
        Ok(self.collection.insert_one(user).await?)
    }
}
