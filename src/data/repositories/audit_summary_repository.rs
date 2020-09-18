use slick_models::AuditSummary;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::{doc, oid::ObjectId};
use wread_data_mongodb::mongodb::{error::Error, results::DeleteResult, Database};

const COLLECTION_NAME: &str = "auditSummaries";

pub async fn get_trend(
    site_id: &String,
    page_id: &String,
    audit_profile_id: &String,
    db: &Database,
) -> Result<Vec<AuditSummary>, Error> {
    let site_id_object = ObjectId::with_string(site_id.as_str()).unwrap();
    let filter =
        doc! {"siteId": site_id_object, "pageId": page_id, "auditProfileId": audit_profile_id};
    crud_repository::find(filter, COLLECTION_NAME, &db).await
}

pub async fn get_by_id(id: &str, db: &Database) -> Result<Option<AuditSummary>, Error> {
    let object_id = ObjectId::with_string(id).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}

pub async fn _delete(id: &str, db: &Database) -> Result<DeleteResult, Error> {
    let summary_id_object = ObjectId::with_string(id).unwrap();
    let filter = doc! {"_id": summary_id_object };
    crud_repository::delete_one(filter, None, COLLECTION_NAME, &db).await
}

pub async fn delete_by_object_id(id: &ObjectId, db: &Database) -> Result<DeleteResult, Error> {
    let filter = doc! {"_id": id };
    crud_repository::delete_one(filter, None, COLLECTION_NAME, &db).await
}
