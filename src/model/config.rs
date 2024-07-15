use std::{fs::File, io::Read, path::PathBuf};

use log::{debug, error};
use serde::Deserialize;

use anyhow::{Error, Result};

#[derive(Deserialize, Debug)]
pub struct CommandConfiguration {
    pub command_name: String,
    pub arguments: Option<Vec<ArgumentEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct ArgumentEntry {
    pub name: String,
    pub required: bool,
}

pub fn get_command_configuration(file: PathBuf) -> Result<CommandConfiguration> {
    debug!("trying to get command config for {file:?}");
    if let Ok(mut config_file) = File::open(file.clone()) {
        let mut config_text = String::new();
        config_file.read_to_string(&mut config_text)?;
        let config: CommandConfiguration = toml::from_str(config_text.as_str())
            .expect("could not deserialize config file: {file:?}");
        debug!("deserialized config file: {:?}", config.command_name);
        Ok(config)
    } else {
        error!("could not open config file: {file:?}");
        Err(Error::msg("could not open config file: {file:?}"))
    }
}
