use axum::{
    routing::{get, post},
    Router,
};
use core::handles;
use log::log::initialize_logger_global;
use tower_http::trace::TraceLayer;
#[tokio::main]
async fn main() {
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("3".to_string())
        .parse::<u8>()
        .unwrap();

    initialize_logger_global(log_level);
    let app = Router::new()
        .route("/upload", get(handles::show_upload))
        .route("/save_image", post(handles::save_image))
        .route("/show_image/:id", get(handles::show_image))
        .route("/download_image/:id", get(handles::download_image))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
