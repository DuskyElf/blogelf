use std::fs;

use crate::{BlogELFError, BlogELFResult};

use super::*;

pub fn new(name: String) -> BlogELFResult {
    fs::create_dir(&name).map_err(|err| -> Box<dyn std::error::Error> {
        format!("Couldn't create new directory '{name}': {err}").into()
    })?;

    init::init(
        std::env::current_dir()
            .map_err(|_| -> BlogELFError { "Access to current directory".into() })?
            .join(name)
            .as_path(),
    )?;

    Ok(())
}
