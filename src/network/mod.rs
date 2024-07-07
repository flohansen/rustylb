use std::{net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};
use tokio::{io::copy_bidirectional, net::{TcpListener, TcpStream}, sync::Mutex};

pub struct Target {
    ip: IpAddr,
    port: u16,
}

impl Target {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Target { ip, port }
    }
}

pub trait BalancingStrategy: Send + Sync {
    fn next(&mut self) -> Option<&Target>;
}

pub struct LoadBalancer {
    strategy: Arc<Mutex<dyn BalancingStrategy>>,
}

impl LoadBalancer {
    pub fn new(strategy: impl BalancingStrategy + 'static) -> Self {
        LoadBalancer { strategy: Arc::new(Mutex::new(strategy)) }
    }

    async fn handle_connection(&self, mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target) = self.strategy.lock().await.next() {
            let addr = SocketAddr::from((target.ip, target.port));
            let mut conn = TcpStream::connect(addr).await?;

            let timeout_duration = Duration::from_secs(5);
            tokio::select! {
                res = copy_bidirectional(&mut socket, &mut conn) => {
                    if let Err(err) = res {
                        eprintln!("Proxy error: {}", err);
                    }
                }

                _ = tokio::time::sleep(timeout_duration) => {
                    eprintln!("Connection timed out");
                }
            }

            Ok(())
        } else {
            Err("No targets".into())
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
