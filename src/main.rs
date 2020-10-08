mod auth;
mod data;
mod handlers;
mod models;

use config;

use data::slick_db;
use env_logger;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use warp::Filter;
use wread_data_mongodb::mongodb::Database;
//use slick_models::{PageScoreParameters, ScoreParameters, SiteScoreParameters};

#[derive(Deserialize, Serialize, Debug)]
struct ApiConfig {
    amqp_uri: String,
    score_queue_name: String,
    db_uri: String,
    db_name: String,
    api_key: String,
    saju_api_key: String,
    saju_firebase_auth_api_base_url: String,
}

/*#[derive(Deserialize, Serialize, Debug)]
struct QueueResponse {
    code: i16,
    message: String,
}*/

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting Slick API..");

    let mut raw_config = config::Config::default();
    raw_config
        .merge(config::Environment::with_prefix("SLICK"))
        .unwrap();
    let api_config = raw_config.try_into::<ApiConfig>().unwrap();
    let api_key = string_to_static_str(api_config.api_key);

    let db = slick_db::get_db(api_config.db_uri.as_str(), api_config.db_name.as_str()).await;

    /*let amqp_addr = api_config.amqp_uri;
    let conn = Connection::connect(
        &amqp_addr,
        ConnectionProperties::default().with_default_executor(8),
    )
    .await
    .unwrap();
    let channel = conn.create_channel().await.unwrap();*/

    let ping = warp::path("ping").map(|| format!("pong"));

    /*let queue_page = warp::post()
        .and(warp::path("queue-page"))
        .and(with_amqp(channel.clone()))
        .and(warp::body::json())
        .and_then(handlers::queue_page_post_handler);

    let queue_site = warp::post()
        .and(warp::path("queue-site"))
        .and(with_amqp(channel.clone()))
        .and(warp::body::json())
        .and_then(handlers::queue_site_post_handler);*/

    let trend = warp::path("trend")
        .and(warp::get())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::trend_get_handler);

    let reports = warp::path("reports")
        .and(warp::get())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::reports_get_handler);

    let reports_delete = warp::path("reports")
        .and(warp::delete())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::reports_delete_handler);

    let summaries_delete = warp::path("summaries")
        .and(warp::delete())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::summaries_delete_handler);

    let sites = warp::path("sites")
        .and(warp::get())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::sites_get_handler);

    let groups = warp::path("group-sites")
        .and(warp::get())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handlers::group_sites_get_handler);

    let register = warp::path("register")
        .and(warp::post())
        .and(warp::header::exact("Api-Key", api_key))
        .and(warp::header("uid"))
        .and(warp::body::json())
        .and(with_firebase_auth_url(api_config.saju_firebase_auth_api_base_url))
        .and(with_saju_api_key(api_config.saju_api_key))
        .and(with_db(db.clone()))
        .and_then(handlers::register_handler);

    let port = env::var("PORT").unwrap_or("8080".into());
    let server_port = format!("0.0.0.0:{}", port);
    let addr = server_port.parse::<SocketAddr>().unwrap();

    let routes = ping
        /*.or(queue_page)
        .or(queue_site)*/
        .or(trend)
        .or(reports)
        .or(reports_delete)
        .or(summaries_delete)
        .or(sites)
        .or(groups)
        .or(register);

    println!("Listening on {}", &addr);

    warp::serve(routes).run(addr).await;
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/*
fn with_amqp(channel: Channel) -> impl Filter<Extract = (Channel,), Error = Infallible> + Clone {
    warp::any().map(move || channel.clone())
}
*/
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_firebase_auth_url(url: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || url.clone())
}

fn with_saju_api_key(key: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || key.clone())
}
