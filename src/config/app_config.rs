use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};

const CONFIG_FILE_PATH: &str = "config_file_path";
const CONFIG_FILE_NAME: &str = "config.toml";
const AWS_CREDENTIALS_FILE: &str = ".aws";

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AWSConfig {
    pub file_path: PathBuf,
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
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub aws: AWSConfig,
    pub logging: LoggingConfig,
    pub performance: String,
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

    fig.extract::<AppConfig>()
        .with_context(|| "Cannot create app config".to_string())
}

fn create_default_values() -> AppConfig {
    AppConfig {
        aws: AWSConfig {
            file_path: get_default_aws_credential_path(),
            ..Default::default()
        },
        logging: LoggingConfig {
            level: "WARN".to_string(),
            ..Default::default()
        },
        performance: "".into(),
    }
}

fn get_default_config_path() -> PathBuf {
    let application_name = env!("CARGO_PKG_NAME");

    let mut config_path = match ProjectDirs::from("com", "tombersoft", application_name) {
        None => PathBuf::from("."),
        Some(project_dirs) => project_dirs.config_dir().to_path_buf(),
    };

    config_path.push(CONFIG_FILE_NAME);
    config_path
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
mod tests {

    use crate::config::command::{CREDENTIALS_KEY, LOG_FILE_PATH, LOG_LEVEL_KEY};

    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn test_default_config() {
        let commands = HashMap::new();

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.aws.file_path, get_default_aws_credential_path());
        assert_eq!(app_config.aws.profiles, Vec::<String>::new());
        assert_eq!(app_config.aws.regions, Vec::<String>::new());
        assert_eq!(app_config.aws.services, Vec::<String>::new());
        assert_eq!(app_config.aws.endpoint, "");
        assert_eq!(app_config.logging.level, "WARN");
        assert_eq!(app_config.logging.log_file_path, "");
        assert_eq!(app_config.performance, "");
    }

    #[test]
    fn test_aws_credentials_path_command_config() {
        let mut commands = HashMap::<String, String>::new();
        let credentials_path = "my_special_credentials";
        commands.insert(CREDENTIALS_KEY.to_string(), credentials_path.to_string());

        let app_config = create_config(&commands).unwrap();

        assert_eq!(app_config.aws.file_path, PathBuf::from(credentials_path));
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
    fn test_config_file() {
        let config_file = r#"
    performance = "yes"

    [aws]
    credentialsPath = "/home/test"
    profiles = ["dev", "prod", "staging"]
    regions = ["eu-central-1", "eu-central-2"]
    services = ["ECS", "EKS", "S3"]
    endpoint = "localhost:4565"

    [logging]
    level = "DEBUG"
    logFilePath = "/home/logs"

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

        assert_eq!(app_config.aws.file_path.to_str().unwrap(), "/home/test");
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
        assert_eq!(app_config.performance, "yes");

        temp_dir.close().unwrap();
    }
}
