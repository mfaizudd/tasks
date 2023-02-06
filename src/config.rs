use config::{Config, File};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}


pub fn get_config(environment: &str) -> Result<Settings> {
    let environment_file = format!("configuration/{}.yml", environment);
    let settings = Config::builder()
        .add_source(File::with_name("configuration/base.yml"))
        .add_source(File::with_name(&environment_file).required(false))
        .build()?
        .try_deserialize::<Settings>()?;
    Ok(settings)
}
