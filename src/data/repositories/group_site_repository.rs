use slick_models::GroupSite;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::doc;
use wread_data_mongodb::mongodb::bson::oid::ObjectId;
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "groupSites";

pub async fn get_by_group_id(id: &String, db: &Database) -> Result<Vec<GroupSite>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    let filter_doc = doc! { "groupId": object_id };
    let sort_doc = doc! { "siteName": 1 };
    crud_repository::find_with_sort(filter_doc, sort_doc, COLLECTION_NAME, &db).await
}
