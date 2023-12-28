#![feature(future_join)]
#![feature(once_cell_try)]

use config::Config;
use log::error;
use ::axum::serve;
use tokio::net::TcpListener;

mod routes;
mod health;
mod config;
mod data_stores;
mod util;
mod startup;
mod files;
mod auth;

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