use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};

use super::command::CONFIG_FILE_PATH;

const CONFIG_FILE_NAME: &str = "config.toml";
const AWS_CREDENTIALS_FILE: &str = ".aws/credentials";

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AWSConfig {
    pub profile: String,
    pub credentials_path: PathBuf,
    pub profiles: Vec<String>,
    pub regions: Vec<String>,
    pub services: Vec<String>,
    pub endpoint: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LoggingConfig {
    pub level: String,
    pub log_file_path: String,
    pub log_to_console: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub aws: AWSConfig,
    pub logging: LoggingConfig,
}

pub fn create_config(arguments: &HashMap<String, String>) -> Result<AppConfig> {
    let app_config_default = create_default_values();

    let mut fig = Figment::new()
        .merge(Serialized::defaults(app_config_default))
        .merge(Toml::file(match arguments.contains_key(CONFIG_FILE_PATH) {
            true => {
                let mut config_file_path = PathBuf::from(arguments.get(CONFIG_FILE_PATH).unwrap());
                config_file_path.push(CONFIG_FILE_NAME);
                config_file_path
            }
            false => get_default_config_path(),
        }))
        .merge(Toml::file(CONFIG_FILE_NAME));

    for (key, value) in arguments {
        fig = fig.merge(Serialized::default(key, value));
    }

    return fig
        .extract::<AppConfig>()
        .with_context(|| format!("Cannot create app config"));
}

fn create_default_values() -> AppConfig {
    return AppConfig {
        aws: AWSConfig {
            credentials_path: get_default_aws_credential_path(),
            ..Default::default()
        },
        logging: LoggingConfig {
            level: "WARN".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
}

fn get_default_config_path() -> PathBuf {
    let application_name = env!("CARGO_PKG_NAME");

    let mut config_path = match ProjectDirs::from("com", "tombersoft", application_name) {
        None => PathBuf::from("."),
        Some(project_dirs) => project_dirs.config_dir().to_path_buf(),
    };

    config_path.push(CONFIG_FILE_NAME);
    return config_path;
}

fn get_default_aws_credential_path() -> PathBuf {
    let mut credential_path = match UserDirs::new() {
        None => PathBuf::from("."),
        Some(user_dirs) => user_dirs.home_dir().to_path_buf(),
    };

    credential_path.push(AWS_CREDENTIALS_FILE);
    credential_path
}

#[cfg(test)]
mod tests_config {

    use crate::config::command::{CONSOLE_KEY, CREDENTIALS_KEY, LOG_FILE_PATH, LOG_LEVEL_KEY};

    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn test_default_config() {
        let commands = HashMap::new();

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.aws.profile, "");
        assert_eq!(
            app_config.aws.credentials_path,
            get_default_aws_credential_path()
        );
        assert_eq!(app_config.aws.profiles, Vec::<String>::new());
        assert_eq!(app_config.aws.regions, Vec::<String>::new());
        assert_eq!(app_config.aws.services, Vec::<String>::new());
        assert_eq!(app_config.aws.endpoint, "");
        assert_eq!(app_config.logging.level, "WARN");
        assert_eq!(app_config.logging.log_file_path, "");
        assert_eq!(app_config.logging.log_to_console, "");
    }

    #[test]
    fn test_aws_credentials_path_command_config() {
        let mut commands = HashMap::<String, String>::new();
        let credentials_path = "my_special_credentials";
        commands.insert(CREDENTIALS_KEY.to_string(), credentials_path.to_string());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(
            app_config.aws.credentials_path,
            PathBuf::from(credentials_path)
        );
    }

    #[test]
    fn test_log_level_command_config() {
        let mut commands = HashMap::<String, String>::new();
        let log_level = "ERROR";
        commands.insert(LOG_LEVEL_KEY.to_string(), log_level.to_string());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.logging.level, log_level);
    }

    #[test]
    fn test_log_file_command_config() {
        let mut commands = HashMap::<String, String>::new();
        let log_file_path = "test/";
        commands.insert(LOG_FILE_PATH.to_string(), log_file_path.to_string());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.logging.log_file_path, log_file_path);
    }

    #[test]
    fn test_log_to_console_command_config() {
        let mut commands = HashMap::<String, String>::new();
        let log_to_console = "yes";
        commands.insert(CONSOLE_KEY.to_string(), log_to_console.to_string());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.logging.log_to_console, log_to_console);
    }

    #[test]
    fn test_config_file() {
        let config_file = r#"
    [aws]
    profile = "dev"
    credentialsPath = "/home/test"
    profiles = ["dev", "prod", "staging"]
    regions = ["eu-central-1", "eu-central-2"]
    services = ["ECS", "EKS", "S3"]
    endpoint = "localhost:4565"

    [logging]
    level = "DEBUG"
    logFilePath = "/home/logs"
    logToConsole = "yes"

        "#
        .to_string();

        let temp_dir = assert_fs::TempDir::new().unwrap();
        temp_dir
            .child(CONFIG_FILE_NAME)
            .write_str(&config_file)
            .unwrap();

        let mut commands = HashMap::<String, String>::new();
        let config_file_path = temp_dir
            .path()
            .as_os_str()
            .to_os_string()
            .into_string()
            .unwrap();
        commands.insert("config_file_path".to_string(), config_file_path.clone());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.aws.profile, "dev");
        assert_eq!(
            app_config.aws.credentials_path.to_str().unwrap(),
            "/home/test"
        );
        assert_eq!(
            app_config.aws.profiles.as_slice(),
            ["dev", "prod", "staging"]
        );
        assert_eq!(
            app_config.aws.regions.as_slice(),
            ["eu-central-1", "eu-central-2"]
        );
        assert_eq!(app_config.aws.services.as_slice(), ["ECS", "EKS", "S3"]);
        assert_eq!(app_config.aws.endpoint, "localhost:4565");
        assert_eq!(app_config.logging.level, "DEBUG");
        assert_eq!(app_config.logging.log_file_path, "/home/logs");
        assert_eq!(app_config.logging.log_to_console, "yes");

        temp_dir.close().unwrap();
    }
}
