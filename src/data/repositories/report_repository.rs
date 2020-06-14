use crate::models::PageScoreReport;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::{Database, error::Error};
use wread_data_mongodb::mongodb::bson::oid::ObjectId;

const COLLECTION_NAME: &str = "reports";

pub async fn get_by_report_id(
    report_id: &String,
    db: &Database,
) -> Result<Option<PageScoreReport>, Error> {
    let object_id = ObjectId::with_string(report_id.as_str()).unwrap();
    crud_repository::find_by_id(
        &object_id,
        COLLECTION_NAME,
        &db,
    ).await
}
