use crate::config::{command::parse_commands, logging::init_tracing};

use anyhow::Result;
use config::config::{create_config, AppConfig};
use std::collections::HashMap;
use tracing::{event, span, Level};
use tui::app::App;

mod config;
mod repository;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let commands = parse_commands();
    let app_config = create_config(&commands)?;
    print!("{:?}\n\n", app_config);

    // Logging can only be initialized after we fetched the configuration parameters.
    let _guard = init_tracing(&app_config.logging);
    log_commands_and_config(&commands, &app_config);

    // let sdk_config = create_aws_config(&app_config.aws).await;
    // event!(Level::DEBUG, "Config {:?}", sdk_config);

    App::run()
}

fn log_commands_and_config(commands: &HashMap<String, String>, app_config: &AppConfig) {
    let span = span!(Level::DEBUG, "Configuration");
    let _guard = span.enter();

    let commands_json = serde_json::to_string(&commands).unwrap();
    event!(Level::DEBUG, "CLI Commands {}", commands_json);

    event!(Level::DEBUG, "{:?}", app_config)
}
