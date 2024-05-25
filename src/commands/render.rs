use markdown;
use std::collections::HashMap;

use crate::{BlogELFError, BlogELFResult};

use super::*;

pub fn render(input: &str, output: &str, template: &str) -> BlogELFResult {
    let template_src_file = std::fs::read_to_string(&template).map_err(|err| -> BlogELFError {
        format!("Couldn't open file '{template}': {err}").into()
    })?;

    let src = std::fs::read_to_string(input).map_err(|err| -> BlogELFError {
        format!("Couldn't read input '{input}': {err}").into()
    })?;

    std::fs::write(
        output,
        template::template_(
            &template_src_file,
            HashMap::from([
                ("title".to_string(), "new_blog".to_string()),
                ("body".to_string(), markdown::to_html(&src)),
            ]),
        ),
    )
    .map_err(|err| -> BlogELFError { format!("Couldn't write output '{output}': {err}").into() })?;

    Ok(())
}
