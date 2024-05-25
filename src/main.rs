use clap::Parser;

mod commands;

pub type BlogELFError = Box<dyn std::error::Error>;
pub type BlogELFResult = Result<(), BlogELFError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    /// Create a new project in an existing directory
    Init,

    /// Create a new project in a new directory
    New { name: String },

    /// Show a field from config
    Config { field: String },

    /// Render the project into a static site
    Render {
        /// The markdown file
        input: String,

        /// The output html file
        output: String,

        /// Template html file
        template: String,
    },

    /// Create a new blog post template
    Template {
        // TODO: add doc comments
        input: String,
        output: String,

        /// <key>=<value> pairs to replace in the template
        args: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    if let Err(e) = match args.subcmd {
        SubCommand::Init => {
            commands::init(&std::env::current_dir().expect("could not get current directory"))
        }

        SubCommand::New { name } => commands::new(name),

        SubCommand::Config { field } => commands::config(&field),

        SubCommand::Render {
            input,
            output,
            template,
        } => commands::render(&input, &output, &template),

        SubCommand::Template {
            input,
            output,
            args,
        } => commands::template(&input, &output, args),
    } {
        eprintln!("[Error]: {e}");
    }
}
