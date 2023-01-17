/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use axum::{http::StatusCode, response::IntoResponse, routing::get_service};
use scsys::AsyncResult;
use tower_http::services::ServeDir;

/// Quickstart a server for static assets
pub async fn wasm_server(path: &str, port: Option<u16>) -> AsyncResult {
    let serve_dir = get_service(ServeDir::new(path)).handle_error(handle_error);
    let app = axum::Router::new().nest_service("/", serve_dir);
    axum::Server::bind(&std::net::SocketAddr::from((
        [0, 0, 0, 0],
        port.unwrap_or(8080),
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();
    Ok(())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
