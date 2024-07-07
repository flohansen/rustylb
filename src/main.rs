use std::net::IpAddr;

pub mod application;
pub mod network;
pub mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hard coded targets to test the load balancer locally.
    // TODO: Read configuration file or something.
    let targets = vec![
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3002),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3003),
    ];

    // The strategy used for distributing incoming TCP packets to the targets.
    let strategy = strategy::RoundRobin::new(targets);

    // Create and run the load balancer.
    // TODO: Prettify this e.g. by using builder pattern.
    network::LoadBalancer::new(strategy)
        .run()
        .await
}
