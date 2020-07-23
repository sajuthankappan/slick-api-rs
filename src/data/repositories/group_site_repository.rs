use slick_models::GroupSite;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::doc;
use wread_data_mongodb::mongodb::bson::oid::ObjectId;
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "groupSites";

pub async fn get_by_group_id(id: &String, db: &Database) -> Result<Vec<GroupSite>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    let doc = doc! { "groupId": object_id };
    crud_repository::find(doc, COLLECTION_NAME, &db).await
}
