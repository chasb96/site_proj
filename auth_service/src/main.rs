#![feature(once_cell_try)]

use config::Config;
use log::error;
use ::axum::serve;
use tokio::net::TcpListener;

mod users;
mod web;
mod config;
mod startup;
mod authorizer;
mod data_stores;
mod routes;
mod axum;
mod util;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::from_env()
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    startup::on_start(&config)
        .await
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