use kube::Client;

mod api;
mod model;
mod storage;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = storage::Dummy;
    let client = Client::try_default().await?;

    let api = self::api::State::init(storage, client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, api).await?;
    Ok(())
}
