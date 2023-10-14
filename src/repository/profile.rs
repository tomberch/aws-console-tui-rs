use crate::config::config::AWSConfig;
use anyhow::{Context, Result};
use regex::Regex;
use std::{collections::HashSet, env, ffi::OsStr, fs};

const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY: &str = "AWS_ACCESS_KEY_ID";
const AWS_DEFAULT_REGION: &str = "AWS_DEFAULT_REGION";

pub fn get_available_profiles(aws_config: &AWSConfig) -> Result<Vec<String>> {
    let mut profiles = Vec::new();

    if let Some(env_profile) = get_profile_from_env() {
        profiles.push(env_profile);
    };

    let credentials = get_credentials(aws_config.credentials_path.as_os_str())?;
    profiles.append(&mut extract_profiles_from_credentials(&credentials)?);

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

fn extract_profiles_from_credentials(credentials: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r"(?m)^\[\w*\]$").context("Cannot parse regex expression")?;
    let hash_matches: HashSet<&str> = regex
        .find_iter(credentials)
        .map(|matches| remove_first_and_last_char(matches.as_str()))
        .collect();

    let temp: Vec<&str> = hash_matches.into_iter().collect();
    Ok(temp.iter().map(|v| v.to_string()).collect())
}

fn get_credentials(credentials_path_os_str: &OsStr) -> Result<String, anyhow::Error> {
    fs::read_to_string(credentials_path_os_str).context(format!(
        "Could not read credentials file {:?}",
        credentials_path_os_str
    ))
}

fn remove_first_and_last_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
