use std::net::IpAddr;

pub mod application;
pub mod network;
pub mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = vec![
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3002),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3003),
    ];

    let strategy = strategy::RoundRobin::new(targets);

    network::LoadBalancer::new(Box::new(strategy))
        .run()
        .await
}
