use app_state::AppState;
use config::Config;
use log::error;
use axum::serve;
use tokio::net::TcpListener;

mod routes;
mod health;
mod config;
mod app_state;
mod data_store;
mod users;
mod nodes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::from_env()
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    let app_state = AppState::try_from(config)
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    let router = routes::routes(app_state);

    serve(listener, router)
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();
}