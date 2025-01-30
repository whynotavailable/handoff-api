use crate::models::AppState;
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn main_routes() -> Router<AppState> {
    Router::new().layer(CorsLayer::permissive())
}
