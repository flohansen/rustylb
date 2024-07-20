use std::{net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};
use tokio::{io::copy_bidirectional, net::{TcpListener, TcpStream}, sync::Mutex};

/// Represents a target server for load balancing.
pub struct Target {
    ip: IpAddr,
    port: u16,
}

impl Target {
    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Target {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Target { ip, port }
    }
}

/// Defines the interface for a load balancing strategy.
pub trait BalancingStrategy: Send + Sync {
    /// Selects the next target server based on the implemented strategy.
    fn next(&mut self) -> Option<&Target>;
}

/// A network load balancer that distributes incoming TCP connections to backend servers according
/// to a chosen balancing strategy.
pub struct LoadBalancer {
    timeout: Duration,
    strategy: Arc<Mutex<dyn BalancingStrategy>>,
}

impl LoadBalancer {
    /// Creates a new `LoadBalancer` instance with the specified balancing strategy.
    pub fn new(strategy: impl BalancingStrategy + 'static) -> Self {
        let timeout = Duration::from_millis(6000);
        Self { timeout, strategy: Arc::new(Mutex::new(strategy)) }
    }

    /// Copies the content of the TCP stream to the given destination and writes the response to
    /// the given source.
    async fn copy_request(&self, timeout_duration: Duration, mut source: TcpStream, mut destination: TcpStream) {
        tokio::select! {
            res = copy_bidirectional(&mut source, &mut destination) => {
                if let Err(err) = res {
                    eprintln!("Proxy error: {}", err);
                }
            }

            _ = tokio::time::sleep(timeout_duration) => {
                eprintln!("Connection timed out");
            }
        }
    }

    /// Handles an incoming TCP connection by forwarding it to a backend server selected by the
    /// load balancing strategy.
    async fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target) = self.strategy.lock().await.next() {
            let addr = SocketAddr::from((target.ip, target.port));
            let target_stream = TcpStream::connect(addr).await?;

            self.copy_request(self.timeout, stream, target_stream).await;

            Ok(())
        } else {
            Err("No targets".into())
        }
    }

    /// Starts the load balancer, listening on the specified address and accepting incoming
    /// connections.
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

pub struct LoadBalancerBuilder {
    timeout: Duration,
    strategy: Option<Arc<Mutex<dyn BalancingStrategy>>>,
}

impl LoadBalancerBuilder {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            strategy: None,
        }
    }

    pub fn strategy(mut self, strategy: Arc<Mutex<dyn BalancingStrategy>>) -> Self {
        self.strategy = Some(strategy);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> LoadBalancer {
        LoadBalancer {
            timeout: self.timeout,
            strategy: self.strategy.unwrap(),
        }
    }
}
