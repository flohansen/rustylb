pub struct LoadBalancer {
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer {}
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
