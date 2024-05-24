use std::collections::HashMap;

use crate::{BlogELFError, BlogELFResult};

pub(crate) fn template_(src: &str, map: HashMap<&str, &str>) -> String {
    let mut src = src.to_string();
    for (k, v) in map {
        src = src.replace(&format!("${{{k}}}"), &v);
    }

    src
}

pub fn template(input: &str, output: &str, args: Vec<String>) -> BlogELFResult {
    let map = args
        .iter()
        .map(|v| {
            v.split_once('=')
                .map(|(a, b)| (&*a.to_string().leak(), &*b.to_string().leak()))
        })
        .collect::<Option<HashMap<_, _>>>()
        .ok_or("Invalid template variable, use '<var>=<value>' format")?;

    let src = std::fs::read_to_string(input)
        .map_err(|err| -> BlogELFError { format!("Couldn't read input: {err}").into() })?;

    std::fs::write(output, template_(&src, map))
        .map_err(|_| -> BlogELFError { "Couldn't write output".into() })?;

    Ok(())
}
