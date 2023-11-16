use crate::config::{command::parse_commands, logging::init_tracing};

use anyhow::Result;

use config::app_config::{create_config, AppConfig};
use state::manager::StateManager;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use tracing::{event, span, Level};
use ui::manager::UIManager;

mod config;
mod repository;
mod state;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let commands = parse_commands();
    let app_config = create_config(&commands)?;

    // Logging can only be initialized after we fetched the configuration parameters.
    let _guard = init_tracing(&app_config.logging);
    log_commands_and_config(&commands, &app_config);

    // Loop Startup
    let cancellation_token = CancellationToken::new();
    let (state_store, state_rx) = StateManager::new(app_config);
    let (ui_manager, action_rx) = UIManager::new();

    tokio::try_join!(
        state_store.run(action_rx, cancellation_token.clone()),
        ui_manager.run(state_rx, cancellation_token.clone()),
    )?;

    Ok(())
}

fn log_commands_and_config(commands: &HashMap<String, String>, app_config: &AppConfig) {
    let span = span!(Level::DEBUG, "Configuration");
    let _guard = span.enter();

    let commands_json = serde_json::to_string(&commands).unwrap();
    event!(Level::DEBUG, "CLI Commands {}", commands_json);

    event!(Level::DEBUG, "{:?}", app_config)
}
