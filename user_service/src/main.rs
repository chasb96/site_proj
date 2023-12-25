#![feature(future_join)]
#![feature(once_cell_try)]

use config::Config;
use log::error;
use ::axum::serve;
use tokio::net::TcpListener;

mod users;
mod routes;
mod startup;
mod health;
mod auth;
mod data_store;
mod util;
mod config;
mod axum;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::from_env()
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    startup::on_start(&config)
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    let router = routes::routes();

    serve(listener, router)
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();
}