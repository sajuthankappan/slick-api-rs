mod data;

use config;
use data::repositories::{
    audit_detail_repository, audit_summary_repository, group_site_repository, site_repository,
};
use data::slick_db;
use env_logger;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use warp::{http::StatusCode, Filter};
use wread_data_mongodb::mongodb::Database;
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

    let trend = warp::path("trend")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(trend_get_handler);

    let reports = warp::path("reports")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(reports_get_handler);

    let reports_delete = warp::path("reports")
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(reports_delete_handler);

    let summaries_delete = warp::path("summaries")
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(summaries_delete_handler);

    let sites = warp::path("sites")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(sites_get_handler);

    let groups = warp::path("group-sites")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(group_sites_get_handler);

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
        .or(groups);

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
async fn trend_get_handler(
    site_id: String,
    page_id: String,
    audit_profile_id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Getting trend for site {}", &site_id);
    let report = audit_summary_repository::get_trend(&site_id, &page_id, &audit_profile_id, &db)
        .await
        .unwrap();
    Ok(warp::reply::json(&report))
}

async fn reports_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Getting report for {}", &id);
    let report = audit_detail_repository::get_by_id(&id, &db).await.unwrap();
    dbg!(&report);
    Ok(warp::reply::json(&report))
}

async fn reports_delete_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Deleting summary for {}", &id);
    let result = audit_detail_repository::delete(&id.as_str(), &db).await;
    if let Err(err) = result {
        log::error!("{}", err);
        Ok(StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn summaries_delete_handler(
    id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Deleting summary for {}", &id);
    let result = audit_summary_repository::get_by_id(id.as_str(), &db).await;
    match result {
        Ok(Some(summary)) => {
            let result = audit_summary_repository::delete_by_object_id(&summary.id().clone().unwrap(), &db).await;
            
            if let Err(err) = result {
                log::error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }
            
            let audit_detail_id = summary.audit_detail_id();
            let result = audit_detail_repository::delete_by_object_id(&audit_detail_id, &db).await;

            if let Err(err) = result {
                log::error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(StatusCode::NO_CONTENT)
        }
        Ok(None) => Ok(StatusCode::NOT_FOUND),
        Err(err) => {
            log::error!("{}", &err);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
    let site = site_repository::get_by_id(&id, &db).await.unwrap();
    Ok(warp::reply::json(&site))
}

async fn group_sites_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Getting sites for group {}", &id);
    let group_sites = group_site_repository::get_by_group_id(&id, &db)
        .await
        .unwrap();
    Ok(warp::reply::json(&group_sites))
}

/*
fn with_amqp(channel: Channel) -> impl Filter<Extract = (Channel,), Error = Infallible> + Clone {
    warp::any().map(move || channel.clone())
}
*/
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
