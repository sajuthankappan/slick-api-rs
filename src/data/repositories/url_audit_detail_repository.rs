use crate::models::UrlAuditDetail;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::oid::ObjectId;
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "urlAuditDetails";

pub async fn get_by_id(
    id: &String,
    db: &Database,
) -> Result<Option<UrlAuditDetail>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}
