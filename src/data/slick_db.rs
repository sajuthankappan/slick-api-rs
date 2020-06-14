use wread_data_mongodb::mongodb::{Client, Database};

pub async fn get_db(db_uri: String, db_name: String) -> Database {
    let client = Client::with_uri_str(db_uri.as_str()).await.unwrap();
    let db = client.database(db_name.as_str());
    db
}
