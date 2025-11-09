use std::{
    marker::{Send, Sync},
    sync::Arc,
};

use axum::{Router, routing::post};

use crate::storage::Storage;

mod build;

pub struct State<S> {
    pub storage: S,
    pub kube_client: kube::Client,
}

impl<S: Send + Sync + Storage + 'static> State<S> {
    pub fn init(storage: S, kube_client: kube::Client) -> Router {
        let s = State {
            storage,
            kube_client,
        };
        Router::new()
            .route("/build", post(build::create))
            .with_state(Arc::new(s))
    }
}
