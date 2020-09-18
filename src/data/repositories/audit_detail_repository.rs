use slick_models::AuditDetail;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::{doc, oid::ObjectId};
use wread_data_mongodb::mongodb::{error::Error, results::DeleteResult, Database};

const COLLECTION_NAME: &str = "auditDetails";

pub async fn get_by_id(id: &String, db: &Database) -> Result<Option<AuditDetail>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}

pub async fn delete(id: &str, db: &Database) -> Result<DeleteResult, Error> {
    let summary_id_object = ObjectId::with_string(id).unwrap();
    let filter = doc! {"_id": summary_id_object };
    crud_repository::delete_one(filter, None, COLLECTION_NAME, &db).await
}

pub async fn delete_by_object_id(id: &ObjectId, db: &Database) -> Result<DeleteResult, Error> {
    let filter = doc! {"_id": id };
    crud_repository::delete_one(filter, None, COLLECTION_NAME, &db).await
}
