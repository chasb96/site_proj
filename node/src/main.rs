use app_state::AppState;
use config::Config;

mod routes;
mod health;
mod config;
mod app_state;
mod data_store;
mod startup;
mod users;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let app_state = AppState::from(config);

    startup::on_start(&app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, routes::routes(app_state)).await.unwrap();
}