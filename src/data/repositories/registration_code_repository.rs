use crate::models::registration::RegistrationCode;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "registrationCodes";

pub async fn get_by_code(code: &str, db: &Database) -> Result<Option<RegistrationCode>, Error> {
    crud_repository::find_one_by_string_field("code", code, COLLECTION_NAME, &db).await
}
