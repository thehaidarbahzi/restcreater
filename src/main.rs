use clap::Parser;
use restcreater::cli::{ Cli, Commands };
use restcreater::commands::{ new, update };

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name, lang, template }) => {
            new::run(name, lang, template).ok();
        }
        Some(Commands::Update) => {
            update::run();
        }
        None => {
            Cli::parse_from(["restcreater", "--help"]);
        }
    }
}
