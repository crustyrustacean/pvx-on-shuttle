// shuttle/src/main.rs

// dependencies
use pavex::config::ConfigLoader;
use pavex::server::Server;
use server::configuration::Profile::{Dev, Prod};
use server_sdk::{ApplicationConfig, ApplicationState};
use shuttle_runtime::{CustomError, SecretStore, Secrets};

// module dependencies
mod shuttle_pavex;

#[shuttle_runtime::main]
async fn pavex(#[Secrets] secrets: SecretStore) -> shuttle_pavex::ShuttlePavex {
    // get the profile from the secrets
    let profile = secrets.get("PX_PROFILE").unwrap_or_default();

    // construct the app profile
    let app_profile = match profile.as_str() {
        "dev" => Dev,
        "prod" => Prod,
        _ => panic!("Unable to set the application profile."),
    };
    tracing::info!("Application profile (set from Secrets): {:?}", app_profile);

    // load the application configuration
    let app_config: ApplicationConfig =
        ConfigLoader::new()
            .profile(app_profile)
            .load()
            .map_err(|err| {
                let error_msg = format!("Unable to load the application configuration: {}", err);
                CustomError::new(err).context(error_msg)
            })?;
    tracing::info!("Application configuration loaded: {:?}", app_config);

    // build the application state
    let app_state = ApplicationState::new(app_config).await.map_err(|err| {
        let error_msg = format!("Unable to build the application state: {}", err);
        CustomError::new(err).context(error_msg)
    })?;
    tracing::info!("Application state built...");

    // build the app server
    let app_server = Server::new();
    tracing::info!("Server built...");

    // build the Shuttle Pavex service, pass the app server and app state
    let shuttle_px = shuttle_pavex::PavexService {
        server: app_server,
        state: app_state,
    };
    tracing::info!("Starting the Shuttle Pavex Service...");

    Ok(shuttle_px)
}
