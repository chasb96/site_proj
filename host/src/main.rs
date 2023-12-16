mod app_state;
mod handlebars;
mod routes;
mod index;
mod assets;


#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, routes::routes()).await.unwrap();
}
