use slick_models::{MetaSite, Site};
use wread_mongodb::mongodb::bson::oid::ObjectId;
use wread_mongodb::mongodb::{error::Error, Database};
use wread_mongodb::{
    crud_repository,
    mongodb::{
        bson::doc,
        results::{InsertOneResult, UpdateResult},
    },
};

const COLLECTION_NAME: &str = "sites";

pub async fn get_by_id(id: &String, db: &Database) -> Result<Option<Site>, Error> {
    let object_id = ObjectId::with_string(id.as_str()).unwrap();
    crud_repository::find_by_id(&object_id, COLLECTION_NAME, &db).await
}

pub async fn get_by_group_id(group_id: &String, db: &Database) -> Result<Vec<MetaSite>, Error> {
    let group_object_id = ObjectId::with_string(group_id.as_str()).unwrap();
    let filter = doc! {"groupId": group_object_id };
    let sort_doc = doc! { "name": 1 };
    crud_repository::find_with_sort(filter, sort_doc, COLLECTION_NAME, &db).await
}

pub async fn add(site: &Site, db: &Database) -> Result<InsertOneResult, Error> {
    crud_repository::add(site, COLLECTION_NAME, db).await
}

pub async fn update(site: &Site, db: &Database) -> Result<UpdateResult, Error> {
    let id = site.id().as_ref().unwrap();
    let filter = doc! {"_id":  id};
    crud_repository::replace_one(filter, site, None, COLLECTION_NAME, db).await
}
