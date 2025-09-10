// shuttle/src/shuttle_pavex.rs

// dependencies
use pavex::server::Server;
use server_sdk::{ApplicationState, run};
use shuttle_runtime::Error;
use std::net::SocketAddr;

// type declarations
pub type ShuttlePavex = Result<PavexService, Error>;

// A wrapper type for [pavex::server::Server] so we can implement [shuttle_runtime::Service] for it.
pub struct PavexService {
    pub server: Server,
    pub state: ApplicationState,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for PavexService {
    // Takes the router that is returned by the user in their [shuttle_runtime::main] function
    // and binds to an address passed in by shuttle.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let server = self
            .server
            .bind(addr)
            .await
            .expect("Failed to bind the server TCP listener");

        tracing::info!("Starting to listen for incoming requests at {}", addr);

        run(server, self.state).await;

        Ok(())
    }
}
