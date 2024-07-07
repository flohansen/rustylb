use tokio::net::{TcpListener, TcpStream};

pub struct LoadBalancer {
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer {}
    }

    async fn handle_connection(_socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: implement load balancing algorithm
        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:3000";
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(async {
                if let Err(err) = Self::handle_connection(socket).await {
                    eprintln!("Error handling connection: {}", err);
                }
            });
        }
    }
}
