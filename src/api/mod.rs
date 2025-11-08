use axum::{Router, routing::post};

mod build;
mod build_req;

pub struct API {}

impl API {
    pub fn init() -> Router {
        Router::new().route("/build", post(build::create))
    }
}
