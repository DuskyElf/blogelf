use std::{borrow::BorrowMut, env, fs, io};

pub mod init;
pub mod new;
pub mod render;
pub mod template;

pub trait Command {
    fn run(&self, args: env::Args) -> Result<(), &str>;
}

pub enum Input {
    File(fs::File),
    Stdin(io::Stdin),
}

pub enum Output {
    File(fs::File),
    Stdout(io::Stdout),
}

pub fn parse_args_io(
    mut args: env::Args,
) -> Result<
    (
        Input,
        Output,
        Vec<String>, // rest_args
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
        Input::File(
            fs::File::options()
                .read(true)
                .open(&inner)
                .map_err(|err| &*format!("Couldn't open file '{inner}': {err}").leak())?,
        )
    } else {
        Input::Stdin(io::stdin())
    };

    let output = if let Some(inner) = output {
        Output::File(
            fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&inner)
                .map_err(|err| &*format!("Couldn't open file '{inner}': {err}").leak())?,
        )
    } else {
        Output::Stdout(io::stdout())
    };

    Ok((input, output, rest_args))
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.read(buf),
            Self::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.write(buf),
            Self::Stdout(ref mut stdin) => stdin.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stdin) => stdin.flush(),
        }
    }
}
