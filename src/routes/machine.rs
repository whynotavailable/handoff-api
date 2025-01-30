use axum::{routing::get, Router};

use crate::{
    errors::{json_ok, JsonResult},
    models::{AppState, SimpleResponse},
};

async fn status() -> JsonResult<SimpleResponse> {
    json_ok(SimpleResponse::new("Ok"))
}

pub fn machine_routes() -> Router<AppState> {
    Router::new().route("/status", get(status))
}
