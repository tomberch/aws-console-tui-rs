use crate::{config::config::AWSConfig, state::appstate::ProfileSource};
use anyhow::{Context, Result};
use regex::RegexBuilder;
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::OsStr,
    fs,
    path::PathBuf,
};

const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
const AWS_DEFAULT_REGION: &str = "AWS_DEFAULT_REGION";
const AWS_CREDENTIALS_FILE: &str = "credentials";
const AWS_CONFIG_FILE: &str = "config";

pub fn get_available_profiles(aws_config: &AWSConfig) -> Result<HashMap<String, ProfileSource>> {
    let mut profiles = HashMap::new();

    if let Some(env_profile) = get_profile_from_env() {
        profiles.insert(env_profile, ProfileSource::Environment);
    };

    let full_credentials_path = join_full_path(aws_config.file_path.clone(), AWS_CREDENTIALS_FILE);
    if let Ok(credentials) = get_entries(full_credentials_path.as_os_str()) {
        for profile in extract_profiles_from_file(&credentials, r"(?m)\[(.*)\]$")?.into_iter() {
            profiles.insert(profile, ProfileSource::CredentialsFile);
        }
    };

    let full_config_path = join_full_path(aws_config.file_path.clone(), AWS_CONFIG_FILE);
    let _z = full_config_path.as_os_str();
    if let Ok(credentials) = get_entries(full_config_path.as_os_str()) {
        for profile in
            extract_profiles_from_file(&credentials, r"(?m)\[profile (.*)\]\s*")?.into_iter()
        {
            profiles.insert(profile, ProfileSource::ConfigFile);
        }
    };

    Ok(profiles)
}

fn get_profile_from_env() -> Option<String> {
    let access_key = env::var(AWS_ACCESS_KEY_ID).unwrap_or("".into());
    let secret_key = env::var(AWS_SECRET_ACCESS_KEY).unwrap_or("".into());
    let region = env::var(AWS_DEFAULT_REGION).unwrap_or("".into());

    if access_key.is_empty() || secret_key.is_empty() || region.is_empty() {
        None
    } else {
        Some(access_key)
    }
}

fn extract_profiles_from_file(credentials: &str, regex_string: &str) -> Result<Vec<String>> {
    let regex = RegexBuilder::new(regex_string)
        .case_insensitive(true)
        .build()
        .context("Cannot parse regex expression")?;

    let hash_matches: HashSet<&str> = regex
        .captures_iter(credentials)
        .filter_map(|caps| caps.get(1))
        .map(|matches| matches.as_str())
        .collect();

    let temp: Vec<&str> = hash_matches.into_iter().collect();
    Ok(temp.iter().map(|v| v.to_string()).collect())
}

fn get_entries(path_os_str: &OsStr) -> Result<String, anyhow::Error> {
    fs::read_to_string(path_os_str)
        .context(format!("Could not read credentials file {:?}", path_os_str))
}

fn join_full_path(aws_config_path: PathBuf, file_name: &str) -> PathBuf {
    let mut credentials_path = aws_config_path;
    credentials_path.push(file_name);
    credentials_path
}

#[cfg(test)]
mod tests {
    use std::env;

    use assert_fs::prelude::{FileWriteStr, PathChild};

    use crate::{
        config::config::AWSConfig,
        repository::profile::{
            get_available_profiles, AWS_ACCESS_KEY_ID, AWS_CONFIG_FILE, AWS_CREDENTIALS_FILE,
            AWS_DEFAULT_REGION, AWS_SECRET_ACCESS_KEY,
        },
        state::appstate::ProfileSource,
    };

    #[test]
    fn test_complete_env_config() {
        let id = "test_id";
        env::set_var(AWS_ACCESS_KEY_ID, id);
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_DEFAULT_REGION, "any");

        let aws_config = AWSConfig {
            file_path: "test_path".into(),
            ..AWSConfig::default()
        };

        let profiles = get_available_profiles(&aws_config).unwrap();

        assert_eq!(1, profiles.len());
        assert!(matches!(
            profiles.get(id).unwrap(),
            ProfileSource::Environment
        ));
    }

    #[test]
    fn test_incomplete_env_config() {
        let id = "test_id";
        env::set_var(AWS_ACCESS_KEY_ID, id);
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");

        let aws_config = AWSConfig {
            file_path: "test_path".into(),
            ..AWSConfig::default()
        };

        let profiles = get_available_profiles(&aws_config).unwrap();

        assert_eq!(0, profiles.len());
    }

    #[test]
    fn test_credential_file_profile_fetch() {
        env::remove_var(AWS_ACCESS_KEY_ID);
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let credential_file = temp_dir.child(AWS_CREDENTIALS_FILE);
        credential_file
            .write_str(
                r"
[default]
lorem ipsum
lorem ipsum

[dev-test]
lorem ipsum
lorem ipsum",
            )
            .unwrap();

        let aws_config = AWSConfig {
            file_path: temp_dir.to_path_buf(),
            ..AWSConfig::default()
        };

        let profiles = get_available_profiles(&aws_config).unwrap();

        assert_eq!(2, profiles.len());
        assert!(profiles.contains_key(&"default".to_string()));
        assert!(profiles.contains_key(&"dev-test".to_string()));
    }

    #[test]
    fn test_config_file_profile_fetch() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let config_file = temp_dir.child(AWS_CONFIG_FILE);
        config_file
            .write_str(
                r"
[profile default]
lorem ipsum
lorem ipsum

[profile dev]
lorem ipsum
lorem ipsum       
    ",
            )
            .unwrap();

        let aws_config = AWSConfig {
            file_path: temp_dir.to_path_buf(),
            ..AWSConfig::default()
        };

        let profiles = get_available_profiles(&aws_config).unwrap();

        assert_eq!(2, profiles.len());
        assert!(profiles.contains_key(&"default".to_string()));
        assert!(profiles.contains_key(&"dev".to_string()));
    }
}
