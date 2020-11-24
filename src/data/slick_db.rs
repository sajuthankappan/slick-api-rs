use wread_mongodb::mongodb::{Client, Database};

pub async fn get_db(db_uri: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(db_uri).await.unwrap();
    let db = client.database(db_name);
    db
}
