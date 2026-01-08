use clap::builder::styling::{ AnsiColor, Effects, Styles };
use clap::{ Parser, Subcommand };

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
}

#[derive(Parser)]
#[command(name = "restcreater")]
#[command(bin_name = "restcreater")]
#[command(version, long_about = None)]
#[command(styles = styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new REST API project
    New {
        #[arg(short, long, help = "Project name", value_name = "NAME")]
        name: Option<String>,
        #[arg(short, long, help = "Project language", requires = "name", value_name = "LANG")]
        lang: Option<String>,
        #[arg(short, long, help = "Project Framework", requires = "lang", value_name = "FRAMEWORK")]
        framework: Option<String>,
        #[arg(
            short,
            long,
            help = "Project template",
            requires = "framework",
            value_name = "TEMPLATE"
        )]
        template: Option<String>,
    },
    /// Update existing project dependencies
    Update,
}
