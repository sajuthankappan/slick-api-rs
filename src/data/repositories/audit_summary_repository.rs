use slick_models::AuditSummary;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::{doc, oid::ObjectId};
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "auditSummaries";

pub async fn get_trend(site_id: &String, page_id: &String, audit_profile_id: &String, db: &Database) -> Result<Vec<AuditSummary>, Error> {
    let site_id_object = ObjectId::with_string(site_id.as_str()).unwrap();
    let filter = doc!{"siteId": site_id_object, "pageId": page_id, "auditProfileId": audit_profile_id};
    crud_repository::find(filter, COLLECTION_NAME, &db).await
}
