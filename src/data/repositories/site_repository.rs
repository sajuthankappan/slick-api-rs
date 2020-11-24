use slick_models::{Site, MetaSite};
use wread_mongodb::{crud_repository, mongodb::bson::doc};
use wread_mongodb::mongodb::bson::oid::ObjectId;
use wread_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "sites";

pub async fn get_by_id(id: &String, db: &Database) -> Result<Option<Site>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}

pub async fn get_by_group_id(group_id: &String, db: &Database) -> Result<Vec<MetaSite>, Error> {
    let group_object_id = ObjectId::with_string(group_id.as_str()).unwrap();
    let filter = doc! {"groupId": group_object_id };
    crud_repository::find(filter, COLLECTION_NAME, &db).await
}
