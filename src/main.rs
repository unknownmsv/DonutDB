mod api;
mod crypto;
mod utils;
mod model;

#[tokio::main]
async fn main() {
    api::start_server().await;
}