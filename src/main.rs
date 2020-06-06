use warp::Filter;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let ping = warp::path!("ping")
        .map(|| format!("pong"));

    let port = env::var("PORT").unwrap_or("8080".into());
    let addr = format!("0.0.0.0:{}", port);
    let server = addr.parse::<SocketAddr>().unwrap();
    let routes = ping;

    println!("Listening on {}", &addr);

    warp::serve(routes)
        .run(server)
        .await;
}
