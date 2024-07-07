use std::{net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};
use tokio::{io::copy_bidirectional, net::{TcpListener, TcpStream}, time::timeout};

pub struct Target {
    ip: IpAddr,
    port: u16,
}

impl Target {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Target { ip, port }
    }
}

pub struct LoadBalancer {
    targets: Vec<Target>,
}

impl LoadBalancer {
    pub fn new(targets: Vec<Target>) -> Self {
        LoadBalancer { targets }
    }

    async fn handle_connection(&self, mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from((self.targets[0].ip, self.targets[0].port));
        let mut conn = TcpStream::connect(addr).await?;

        let timeout_duration = Duration::from_secs(5);
        if let Err(err) = timeout(timeout_duration, copy_bidirectional(&mut socket, &mut conn)).await {
            eprintln!("Error proxying request: {}", err);
        }

        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:3000";
        let listener = TcpListener::bind(addr).await?;

        let arc = Arc::new(self);

        loop {
            let (socket, addr) = listener.accept().await?;

            println!("Received request from {}", addr);

            let lb = arc.clone();
            tokio::spawn(async move {
                if let Err(err) = lb.handle_connection(socket).await {
                    eprintln!("Error handling connection: {}", err);
                }
            });
        }
    }
}
