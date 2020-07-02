use config;
use env_logger;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use warp::Filter;
use wread_data_mongodb::mongodb::Database;

mod data;
use data::repositories::{audit_detail_repository, site_repository, site_tread_repository};
use data::slick_db;
//use slick_models::{PageScoreParameters, ScoreParameters, SiteScoreParameters};

#[derive(Deserialize, Serialize, Debug)]
struct ApiConfig {
    amqp_uri: String,
    score_queue_name: String,
    db_uri: String,
    db_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct QueueResponse {
    code: i16,
    message: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting Slick API..");

    let mut raw_config = config::Config::default();
    raw_config
        .merge(config::Environment::with_prefix("SLICK"))
        .unwrap();
    let api_config = raw_config.try_into::<ApiConfig>().unwrap();

    let db = slick_db::get_db(api_config.db_uri.clone(), api_config.db_name.clone()).await;

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
        .and_then(queue_page_post_handler);

    let queue_site = warp::post()
        .and(warp::path("queue-site"))
        .and(with_amqp(channel.clone()))
        .and(warp::body::json())
        .and_then(queue_site_post_handler);*/

    let reports = warp::path("reports")
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(reports_get_handler);

    let sites = warp::path("sites")
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(sites_get_handler);

    let treads = warp::path("treads")
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(treads_get_handler);

    let site_treads = warp::path("site-treads")
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(treads_for_site_get_handler);

    let port = env::var("PORT").unwrap_or("8080".into());
    let server_port = format!("0.0.0.0:{}", port);
    let addr = server_port.parse::<SocketAddr>().unwrap();

    let routes = ping
        /*.or(queue_page)
        .or(queue_site)*/
        .or(reports)
        .or(treads)
        .or(site_treads)
        .or(sites);

    println!("Listening on {}", &addr);

    warp::serve(routes).run(addr).await;
}
/*
async fn queue_page_post_handler(
    channel: Channel,
    page_score_parameters: PageScoreParameters,
) -> Result<impl warp::Reply, Infallible> {
    let parameters = ScoreParameters {
        page: Some(page_score_parameters),
        site: None,
    };

    send_score_request_to_queue(&channel, &parameters).await;

    let resp = QueueResponse {
        code: 200,
        message: String::from("Queued"),
    };

    Ok(warp::reply::json(&resp))
}

async fn queue_site_post_handler(
    channel: Channel,
    site_score_parameters: SiteScoreParameters,
) -> Result<impl warp::Reply, Infallible> {
    let parameters = ScoreParameters {
        page: None,
        site: Some(site_score_parameters),
    };

    send_score_request_to_queue(&channel, &parameters).await;

    let resp = QueueResponse {
        code: 200,
        message: String::from("Queued"),
    };

    Ok(warp::reply::json(&resp))
}
*/
async fn reports_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Getting report for {}", &id);
    let report = audit_detail_repository::get_by_id(&id, &db).await.unwrap();
    Ok(warp::reply::json(&report))
}

async fn treads_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Getting tread for {}", &id);
    let report = site_tread_repository::get_by_id(&id, &db).await.unwrap();
    Ok(warp::reply::json(&report))
}

async fn treads_for_site_get_handler(
    id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Getting treads for site {}", &id);
    let treads = site_tread_repository::get_by_site_id(&id, &db)
        .await
        .unwrap();
    Ok(warp::reply::json(&treads))
}
/*
async fn send_score_request_to_queue(channel: &Channel, parameters: &ScoreParameters) {
    let payload = serde_json::to_string(&parameters).unwrap();

    channel
        .basic_publish(
            "",
            "score-requests",
            BasicPublishOptions::default(),
            payload.into_bytes(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
}
*/
async fn sites_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Getting site for {}", &id);
    let report = site_repository::get_by_id(&id, &db).await.unwrap();

    Ok(warp::reply::json(&report))
}
/*
fn with_amqp(channel: Channel) -> impl Filter<Extract = (Channel,), Error = Infallible> + Clone {
    warp::any().map(move || channel.clone())
}
*/
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
