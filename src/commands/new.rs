use std::fs;

use super::*;

pub struct New;

impl Command for New {
    fn run(&self, mut args: env::Args) -> Result<(), &str> {
        // TODO: check for correct number of arguments
        let name = args.next().ok_or("No name provided")?;
        fs::create_dir(&name)
            .map_err(|err| &*format!("Couldn't create new directory '{name}': {err}").leak())?;

        init::init(
            env::current_dir()
                .expect("Access to current directory")
                .join(name)
                .as_path(),
        )?;

        Ok(())
    }
}
