use std::collections::HashMap;
use std::env;
use std::process::exit;

mod commands;

use commands::*;

fn main() {
    let result = start();

    if let Err(err) = result {
        eprintln!("Error: {err}");
        exit(-1);
    }
}

fn start() -> Result<(), String> {
    let mut commands: HashMap<&str, &dyn Command> = HashMap::new();
    commands.insert("render", &render::Render);
    commands.insert("template", &template::Template);

    let (command, args) = parse_args_command(&commands)?;
    command.run(args)?;

    Ok(())
}

fn parse_args_command<'cmd>(
    commands: &HashMap<&str, &'cmd dyn Command>,
) -> Result<(&'cmd dyn Command, env::Args), &'static str> {
    let mut args = env::args();
    let _prog_name = args.next().expect("where's program name");

    let Some(command) = args.next() else {
        return Err("No Command Provided");
    };

    let Some(command_obj) = commands.get(&*command) else {
        return Err("Invalid Command");
    };

    Ok((*command_obj, args))
}
