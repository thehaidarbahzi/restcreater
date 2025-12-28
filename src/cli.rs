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
#[command(version, about = "CLI tool to scaffold REST API projects", long_about = None)]
#[command(styles = styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new REST API project
    New {
        /// Project name
        name: Option<String>,
        #[arg(short, long, help = "Programming language (rust, go, etc.)")]
        lang: Option<String>,
        #[arg(short, long, help = "Web framework (axum, warp, gin, etc.)")]
        framework: Option<String>,
        #[arg(short, long, help = "Project template (blank, basic, etc.)")]
        template: Option<String>,
    },
    /// Update existing project dependencies
    Update,
}
