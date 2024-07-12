use std::{
    fs::{DirEntry, File},
    io::Read,
};

use log::{debug, error, info};
use serde::Deserialize;

use anyhow::{Error, Result};

#[derive(Deserialize)]
pub struct CommandConfiguration {
    pub command_name: String,
    pub arguments: Option<Vec<String>>,
}

pub fn get_command_configuration(file: &DirEntry) -> Result<CommandConfiguration> {
    debug!("trying to get command config for {file:?}");
    if let Ok(mut config_file) = File::open(file.path()) {
        let mut config_text = String::new();
        config_file.read_to_string(&mut config_text)?;
        let config: CommandConfiguration = toml::from_str(config_text.as_str())
            .expect("could not deserialize config file: {file:?}");
        Ok(config)
    } else {
        error!("could not open config file: {file:?}");
        Err(Error::msg("could not open config file: {file:?}"))
    }
}
