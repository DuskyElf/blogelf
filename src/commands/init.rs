use include_dir::{include_dir, Dir};
use std::{env, path::Path};

use super::*;

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

pub fn init(path: &Path) -> Result<(), &'static str> {
    let _name = path // will be used in config file
        .file_name()
        .expect("directory name")
        .to_str()
        .ok_or("Directory name is not UTF-8 compliant")?;

    STATIC_DIR
        .extract(path)
        .map_err(|err| &*format!("Couldn't write to directory: {err}").leak())?;

    Ok(())
}

pub struct Init;

impl Command for Init {
    fn run(&self, _args: env::Args) -> Result<(), &str> {
        let pwd = env::current_dir()
            .map_err(|err| &*format!("Couldn't read currect working directory: {err}").leak())?;

        init(&pwd)
    }
}
