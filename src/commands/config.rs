use serde::{Deserialize, Serialize};
use toml;

use std::{collections::HashMap, fs, path::Path};

use super::*;
use crate::{BlogELFError, BlogELFResult};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub globals: HashMap<String, String>,
    pub src_dir: String,
    pub blog_tmpl: String,
    pub index_tmpl: String,
    pub index_build: String,
    pub build_dir: String,
}

pub(crate) fn config_() -> Result<Config, BlogELFError> {
    if !Path::new(init::CONFIG_NAME)
        .try_exists()
        .map_err(|err| -> BlogELFError { format!("Couldn't access files: {err}").into() })?
    {
        return Err(
            "No Blog system in this directory, initialize with `init` or create new with `new` command".into(),
        );
    }

    let config_str = fs::read_to_string(init::CONFIG_NAME).unwrap();
    toml::from_str(&config_str)
        .map_err(|err| -> BlogELFError { format!("While parsing Config file:\n{err}").into() })
}

// TODO: Change config file with commands
pub fn config(field: &str) -> BlogELFResult {
    let config = config_()?;

    match field {
        "src_dir" => println!("{}", config.src_dir),
        "blog_tmpl" => println!("{}", config.blog_tmpl),
        "index_tmpl" => println!("{}", config.index_tmpl),
        "index_build" => println!("{}", config.index_build),
        "artifacts_dir" => println!("{}", config.build_dir),
        _ => return Err("Invalid config field provided".into()),
    };

    Ok(())
}
