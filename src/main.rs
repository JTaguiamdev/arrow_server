#[tokio::main]
async fn main() {
    arrow_server_lib::api::start().await;
}
