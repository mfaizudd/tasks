use std::path::Path;

use anyhow::Result;
use config::{Config, Environment, File};
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct OauthSettings {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub authorization_url: String,
    pub token_url: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub oauth: OauthSettings,
}

pub fn get_config(environment: &str) -> Result<Settings> {
    let base_path = ProjectDirs::from("net", "faizud", "tasks")
        .and_then(|dirs| {
            let config_dir = dirs.config_dir();
            if config_dir.exists() {
                config_dir.to_str().map(|path| path.to_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "configuration".to_owned());

    let base_path = Path::new(&base_path);
    let base_file = base_path.join("base.yml");
    let environment_file = base_path.join(format!("{environment}.yml"));
    let settings = Config::builder()
        .add_source(File::from(base_file))
        .add_source(File::from(environment_file).required(false))
        .add_source(Environment::with_prefix("tasks"))
        .build()?
        .try_deserialize::<Settings>()?;
    Ok(settings)
}

pub fn get_config_from_file(file: &str) -> Result<Settings> {
    let settings = Config::builder()
        .add_source(File::with_name(file))
        .build()?
        .try_deserialize::<Settings>()?;
    Ok(settings)
}
