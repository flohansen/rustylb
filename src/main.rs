use std::{net::IpAddr, time::Duration};

pub mod application;
pub mod network;
pub mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hard coded targets to test the load balancer locally.
    // TODO: Read configuration file and use tests to make sure, the load balancing is working.
    let targets = vec![
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3002),
        network::Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3003),
    ];

    // The strategy used for distributing incoming TCP packets to the targets.
    let round_robin = strategy::RoundRobin::new(targets);

    // Create and run the load balancer.
    let load_balancer = network::LoadBalancerBuilder::new()
        .strategy(round_robin)
        .timeout(Duration::from_secs(60))
        .build();

    load_balancer.run().await
}
