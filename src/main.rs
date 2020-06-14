use config;
use lapin::{
    options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use warp::Filter;

#[derive(Deserialize, Serialize, Debug)]
struct ApiConfig {
    amqp_uri: String,
    score_queue_name: String,
}

#[derive(Deserialize, Serialize)]
struct ScoreParameters {
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct QueueResponse {
    code: i8,
    message: String,
}

#[tokio::main]
async fn main() {
    let mut raw_config = config::Config::default();
    raw_config
        .merge(config::Environment::with_prefix("SLICK"))
        .unwrap();
    let api_config = raw_config.try_into::<ApiConfig>().unwrap();

    let amqp_addr = api_config.amqp_uri;
    let conn = Connection::connect(
        &amqp_addr,
        ConnectionProperties::default().with_default_executor(8),
    )
    .await
    .unwrap();
    let channel = conn.create_channel().await.unwrap();

    let ping = warp::path("ping").map(|| format!("pong"));

    let queue = warp::post()
        .and(warp::path("queue"))
        .and(with_amqp(channel))
        .and(warp::body::json())
        .and_then(queue_post_handler);

    let score = warp::path("report")
        .and(warp::path::param())
        .map(|id: String| warp::reply::json(&id));

    let port = env::var("PORT").unwrap_or("8080".into());
    let server_port = format!("0.0.0.0:{}", port);
    let addr = server_port.parse::<SocketAddr>().unwrap();

    let routes = ping.or(queue).or(score);

    println!("Listening on {}", &addr);

    warp::serve(routes).run(addr).await;
}

async fn queue_post_handler(
    channel: Channel,
    parameters: ScoreParameters,
) -> Result<impl warp::Reply, Infallible> {
    send_page_score_request_to_queue(&channel, &parameters).await;

    let resp = QueueResponse {
        code: 1,
        message: parameters.url,
    };

    Ok(warp::reply::json(&resp))
}

async fn send_page_score_request_to_queue(
    channel: &Channel,
    parameters: &ScoreParameters,
) {
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

fn with_amqp(channel: Channel) -> impl Filter<Extract = (Channel,), Error = Infallible> + Clone {
    warp::any().map(move || channel.clone())
}
