use include_dir::{include_dir, Dir};
use std::{collections::HashMap, fs, path::Path};

use crate::{BlogELFError, BlogELFResult};

use super::*;

const CONFIG_NAME: &str = "BlogElf.toml";
static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

pub fn init(path: &Path) -> BlogELFResult {
    STATIC_DIR
        .extract(&path)
        .map_err(|err| -> BlogELFError { format!("Couldn't write to directory: {err}").into() })?;

    let config = STATIC_DIR
        .get_file(CONFIG_NAME)
        .expect("static directory setup correctly")
        .contents_utf8()
        .expect("UTF-8 compliant");

    let name = path
        .file_name()
        .expect("directory name")
        .to_str()
        .ok_or("Directory name is not UTF-8 compliant")?;

    let config = template::template_(
        config,
        HashMap::from([("name".to_string(), format!("\"{name}\""))]),
    );

    fs::write(path.join(CONFIG_NAME), config).map_err(|err| -> BlogELFError {
        format!("Couldn't write to file '{CONFIG_NAME}': {err}").into()
    })?;

    Ok(())
}
