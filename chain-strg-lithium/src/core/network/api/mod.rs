pub struct ISSNetworking {
    config: ISSNetworkingConfig,
}

pub enum ISSNetworkingConfig {
    Standard,
}


pub struct NetworkingInit;

impl NetworkingInit {
    #[tokio::main]
    pub async fn new() {
        // Initialize Logging
        pretty_env_logger::init();

        
    }
}