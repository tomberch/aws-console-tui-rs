use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use figment::{
    providers::{Format, Serialized, Toml},
    Error, Figment,
};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
#[allow(unused)]
pub struct AppConfig {
    pub profile: String,
    pub aws_credentials_path: PathBuf,
    pub profile_filters: Vec<String>,
    pub region_filters: Vec<String>,
    pub services_filters: Vec<String>,
    pub endpoint: String,
    pub log_level: String,
    pub log_file_path: String,
    pub log_to_console: String,
}

pub fn create_config(arguments: &HashMap<String, String>) -> Result<AppConfig, Error> {
    let app_config_default = create_default_values();

    return Figment::new()
        .merge(Serialized::defaults(app_config_default))
        .merge(Toml::file(get_default_config_path()))
        .merge(Toml::file("config.toml"))
        .merge(Serialized::defaults(arguments))
        .extract();
}

fn create_default_values() -> AppConfig {
    return AppConfig {
        aws_credentials_path: get_default_aws_credential_path(),
        log_level: "WARN".to_string(),
        ..AppConfig::default()
    };
}

fn get_default_config_path() -> PathBuf {
    let application_name = env!("CARGO_PKG_NAME");

    let mut config_path = match ProjectDirs::from("com", "tombersoft", application_name) {
        None => PathBuf::from("."),
        Some(project_dirs) => project_dirs.config_dir().to_path_buf(),
    };

    config_path.push("config.toml");
    return config_path;
}

fn get_default_aws_credential_path() -> PathBuf {
    let mut credential_path = match UserDirs::new() {
        None => PathBuf::from("."),
        Some(user_dirs) => user_dirs.home_dir().to_path_buf(),
    };

    credential_path.push(".aws/credentials");
    credential_path
}
