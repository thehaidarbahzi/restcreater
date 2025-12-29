use inline_colorization::*;
use inquire::InquireError;
use crate::helper::prompts::{ smart_prompts };
use crate::helper::generation::generate_project;

pub fn run(
    name: Option<String>,
    lang: Option<String>,
    framework: Option<String>,
    template: Option<String>
) {
    println!();
    println!("{color_green}{style_bold}RestCreater{style_reset}{color_reset}");
    println!("{color_bright_black}   Scaffold your REST API project in seconds{color_reset}");
    println!();

    match smart_prompts(name, lang, framework, template) {
        Ok(config) => {
            generate_project(&config);
        }
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
            println!();
            println!("{color_yellow}Operation cancelled.{color_reset}");
        }
        Err(e) => {
            eprintln!("{color_red}Error: {}{color_reset}", e);
        }
    }
}
