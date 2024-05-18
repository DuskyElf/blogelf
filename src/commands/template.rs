use std::{collections::HashMap, env};

use super::*;

pub fn template(src: &str, map: HashMap<&str, &str>) -> String {
    let mut src = src.to_string();
    for (k, v) in map {
        src = src.replace(&format!("${{{k}}}"), &v);
    }

    src
}

pub struct Template;

impl Command for Template {
    fn run(&self, args: env::Args) -> Result<(), &str> {
        let (input, output, args) = parse_args_io(args)?;
        let mut input = input.unwrap_or(Box::new(io::stdin()));
        let mut output = output.unwrap_or(Box::new(io::stdout()));

        let map = args
            .iter()
            .map(|v| {
                v.split_once('=')
                    .map(|(a, b)| (&*a.to_string().leak(), &*b.to_string().leak()))
            })
            .collect::<Option<HashMap<_, _>>>()
            .ok_or("Invalid temlpate variable, use '<var>=<value>' format")?;

        let mut src = String::new();
        input
            .read_to_string(&mut src)
            .map_err(|err| &*format!("Couldn't read input: {err}").leak())?;

        writeln!(output, "{}", template(&src, map))
            .map_err(|err| &*format!("Couldn't write output: {err}").leak())?;

        Ok(())
    }
}
