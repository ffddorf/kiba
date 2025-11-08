use k8s_openapi::api::core::v1::{Pod, PodSpec};
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

use crate::api::API;

mod api;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let api = API::init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, api).await.unwrap();

    let pods: Api<Pod> = Api::default_namespaced(client);
    let runner = pods
        .create(
            &PostParams::default(),
            &Pod {
                metadata: ObjectMeta {
                    generate_name: Some("image-builder-".into()),
                    ..Default::default()
                },
                spec: Some(PodSpec {
                    ..Default::default()
                }),
                status: None,
            },
        )
        .await?;

    todo!()
}
