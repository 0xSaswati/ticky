mod db;
mod routes;
mod models;
mod handlers;

use axum::serve;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::init_db().await;

    let app = routes::create_router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Running at http://localhost:8080");
    serve(listener, app).await.unwrap();
}
