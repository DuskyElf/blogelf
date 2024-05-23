use markdown;
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
};

use super::*;

pub fn render(src: &str, template_src: &str) -> String {
    let out = template::template(
        template_src,
        HashMap::from([("title", "new_blog"), ("body", &markdown::to_html(&src))]),
    );

    out
}

pub struct Render;

impl Command for Render {
    fn run(&self, args: env::Args) -> Result<(), &str> {
        let (mut input, mut output, args) = parse_args_io(args)?;

        // TODO: check for correct number of arguments
        let template_src_name = args.iter().next().ok_or("No template file provided")?;
        let template_src_file = fs::read_to_string(&template_src_name)
            .map_err(|err| &*format!("Couldn't open file '{template_src_name}': {err}").leak())?;

        let mut src = String::new();
        input
            .read_to_string(&mut src)
            .map_err(|err| &*format!("Couldn't read input: {err}").leak())?;

        writeln!(output, "{}", render(&src, &template_src_file))
            .map_err(|err| &*format!("Couldn't write output: {err}").leak())?;
        Ok(())
    }
}
