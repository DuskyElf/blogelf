use std::{ffi::OsStr, fs};

use super::*;
use crate::{BlogELFError, BlogELFResult};

pub fn build() -> BlogELFResult {
    let config = config::config_()?;

    let src_dir = fs::read_dir(&config.src_dir).map_err(|err| -> BlogELFError {
        format!("Couldn't read src_dir `{}`: {err}", config.src_dir).into()
    })?;

    fs::create_dir_all(&config.build_dir)?;

    for file in src_dir {
        let path = file?.path();
        if path.extension().unwrap_or_default() != "md" {
            continue;
        }

        // NIGHTLY feature can be used: Path::file_prefix() https://github.com/rust-lang/rust/issues/86319
        let (file_name, _) = path
            .file_name()
            .and_then(OsStr::to_str)
            .and_then(|s| s.rsplit_once('.'))
            .expect("unreachable becaues of continue condition above");

        render::render(
            path.to_str().unwrap(),
            &format!("{}/{}.html", config.build_dir, file_name),
            &config.blog_tmpl,
        )?;
    }

    Ok(())
}
