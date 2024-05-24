use include_dir::{include_dir, Dir};
use std::path::Path;

use crate::{BlogELFError, BlogELFResult};

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

pub fn init(path: &Path) -> BlogELFResult {
    let _ = path
        .file_name()
        .map(|i| i.to_str())
        .ok_or("Directory name is not UTF-8 compliant")?;

    STATIC_DIR
        .extract(path)
        .map_err(|err| -> BlogELFError { format!("Couldn't write to directory: {err}").into() })?;

    Ok(())
}
