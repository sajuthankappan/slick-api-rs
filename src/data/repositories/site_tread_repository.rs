use slick_models::SiteTread;
use wread_data_mongodb::crud_repository;
use wread_data_mongodb::mongodb::bson::doc;
use wread_data_mongodb::mongodb::bson::oid::ObjectId;
use wread_data_mongodb::mongodb::{error::Error, Database};

const COLLECTION_NAME: &str = "siteTreads";

pub async fn get_by_id(id: &String, db: &Database) -> Result<Option<SiteTread>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}

pub async fn get_by_site_id(site_id: &String, db: &Database) -> Result<Vec<SiteTread>, Error> {
    let object_id = ObjectId::with_string(site_id.as_str()).unwrap();
    crud_repository::find(doc! {"siteId": &object_id}, COLLECTION_NAME, &db).await
}
