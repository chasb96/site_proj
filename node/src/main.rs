use app_state::AppState;
use config::Config;
use log::error;

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

    let config = Config::from_env();

    let app_state = AppState::try_from(config)
        .unwrap_or_else(|e| {
            error!("{}", e); 
            panic!("{}", e)
        });


    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, routes::routes(app_state)).await.unwrap();
}