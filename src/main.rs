use std::net::IpAddr;

pub mod application;
pub mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = vec![
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
    ];

    network::LoadBalancer::new(targets)
        .run()
        .await
}
