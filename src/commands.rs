use std::{borrow::BorrowMut, env, fs, io};

pub mod init;
pub mod new;
pub mod render;
pub mod template;

pub trait Command {
    fn run(&self, args: env::Args) -> Result<(), &str>;
}

pub fn parse_args_io(
    mut args: env::Args,
) -> Result<
    (
        Option<Box<dyn io::Read>>,  // input
        Option<Box<dyn io::Write>>, // output
        Vec<String>,                // rest_args
    ),
    &'static str,
> {
    let mut input = None;
    let mut output = None;
    let mut rest_args = Vec::new();
    while let Some(arg) = args.borrow_mut().next() {
        let m = arg.as_str();
        match m {
            "-i" => input = args.next(),
            "-o" => output = args.next(),
            _ => rest_args.push(arg),
        }
    }

    let input = if let Some(inner) = input {
        Some(Box::new(
            fs::File::options()
                .read(true)
                .open(&inner)
                .map_err(|err| &*format!("Couldn't open file '{inner}': {err}").leak())?,
        ) as Box<dyn io::Read>)
    } else {
        None
    };

    let output = if let Some(inner) = output {
        Some(Box::new(
            fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&inner)
                .map_err(|err| &*format!("Couldn't open file '{inner}': {err}").leak())?,
        ) as Box<dyn io::Write>)
    } else {
        None
    };

    Ok((input, output, rest_args))
}
