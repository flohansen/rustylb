pub mod application;
pub mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    network::LoadBalancer::new()
        .run()
        .await
}
