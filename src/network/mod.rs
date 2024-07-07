use std::{net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};
use tokio::{io::copy_bidirectional, net::{TcpListener, TcpStream}, sync::Mutex, time::timeout};

pub struct Target {
    ip: IpAddr,
    port: u16,
}

impl Target {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Target { ip, port }
    }
}

pub trait BalancingStrategy {
    fn next(&mut self) -> Option<&Target>;
}

pub struct LoadBalancer {
    strategy: Box<dyn BalancingStrategy + Send + Sync>,
}

impl LoadBalancer {
    pub fn new(strategy: Box<dyn BalancingStrategy + Send + Sync>) -> Self {
        LoadBalancer { strategy }
    }

    async fn handle_connection(&mut self, mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        match self.strategy.next() {
            Some(target) => {
                let addr = SocketAddr::from((target.ip, target.port));
                let mut conn = TcpStream::connect(addr).await?;

                let timeout_duration = Duration::from_secs(5);
                if let Err(err) = timeout(timeout_duration, copy_bidirectional(&mut socket, &mut conn)).await {
                    eprintln!("Error proxying request: {}", err);
                }

                Ok(())
            }
            None => Ok(())
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:3000";
        let listener = TcpListener::bind(addr).await?;

        let arc = Arc::new(Mutex::new(self));

        loop {
            let (socket, addr) = listener.accept().await?;

            println!("Received request from {}", addr);

            let lb = arc.clone();
            tokio::spawn(async move {
                if let Err(err) = lb.lock().await.handle_connection(socket).await {
                    eprintln!("Error handling connection: {}", err);
                }
            });
        }
    }
}
