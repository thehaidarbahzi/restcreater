use cliclack::{ clear_screen, input, intro, log, note, outro, select, spinner };
use colored::Colorize;
use console::{ style };

use crate::helper::{
    generation::scaffold_project,
    templates::{ check_template, check_lang, get_lang, get_setup_lang, get_template },
    utils::{ check_folder_exists, sanitize_name },
};

pub fn run(
    def_name: Option<String>,
    def_lang: Option<String>,
    def_templt: Option<String>
) -> std::io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    clear_screen()?;

    intro(style(" Restcreater ").on_cyan().black())?;

    let project_name = match def_name {
        Some(name) => {
            let name = sanitize_name(&name);
            log::step(format!("Project name:\n{}", name.bright_black()))?;
            name
        }
        None => {
            let input_name: String = input("Project name:")
                .placeholder("my-project")
                .default_input("my-project")
                .validate(|input: &String| {
                    if check_folder_exists(input.trim()) {
                        Err("Please enter a different project name.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;
            sanitize_name(&input_name)
        }
    };

    let project_lang = match def_lang {
        Some(lang) => {
            if let Some(item) = check_lang(&lang) {
                log::step(format!("Select a programming language:\n{}", item.lang.bright_black()))?;
                item.id.to_string()
            } else {
                let input_lang = select("Select a programming language:")
                    .items(&get_lang())
                    .interact()?;
                input_lang.to_string()
            }
        }
        None => {
            let input_lang = select("Select a programming language:")
                .items(&get_lang())
                .interact()?;
            input_lang.to_string()
        }
    };

    let project_template = match def_templt {
        Some(templt) if check_template(&templt, &project_lang) => {
            log::step(format!("Select a template:\n{}", &templt.bright_black()))?;
            templt
        }
        _ => {
            let input_templt = select("Select a template:")
                .items(&get_template(&project_lang))
                .interact()?;
            input_templt.to_string()
        }
    };

    let spinner = spinner();
    spinner.start(
        format!(
            "Scaffolding project \"{}\" using \"{}\" template",
            &project_name,
            &project_template
        )
    );

    match scaffold_project(&project_lang, &project_template, &project_name) {
        Ok(_) => {
            spinner.stop("Scaffolding process completed.");
            note(
                "Next steps:",
                format!("cd {}\n{}", project_name, get_setup_lang(&project_lang)).as_str()
            )?;
        }
        Err(_) => {
            spinner.error("Something went wrong.");
        }
    }

    outro(
        format!(
            "Problems? {}\n",
            style("https://github.com/thehaidarbahzi/restcreater/issues").cyan().underlined()
        )
    )?;

    Ok(())
}
