use cliclack::{ input, intro, log, note, outro, select, set_theme, spinner };
use colored::Colorize;
use console::style;

use crate::helper::{
    generation::scaffold_project,
    query::{ check_lang, check_template, get_framework, get_lang, get_templates },
    registry::scan_templates,
    theme::CustomTheme,
    utils::{ check_folder_exists, sanitize_name, to_select_items },
};

pub fn run(
    def_name: Option<String>,
    def_lang: Option<String>,
    def_framework: Option<String>,
    def_templt: Option<String>
) -> std::io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    set_theme(CustomTheme);

    intro(style(" Restcreater (Esc to Exit) ").bold().on_green())?;

    let registry = scan_templates();

    let project_name = match def_name {
        Some(name) => {
            let name = sanitize_name(&name);
            log::step(format!("Project name:\n{}", name.bright_black()))?;
            name
        }
        None => {
            let input: String = input("Project name:")
                .placeholder("my-project")
                .default_input("my-project")
                .validate(|v: &String| {
                    if check_folder_exists(v.trim()) {
                        Err("Folder already exists")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;
            sanitize_name(&input)
        }
    };

    let langs = get_lang(&registry);

    let project_lang = match def_lang {
        Some(lang) if check_lang(&registry, &lang) => {
            log::step(format!("Language:\n{}", lang.bright_black()))?;
            lang
        }
        _ => {
            select("Select a programming language:")
                .items(&to_select_items(&langs))
                .interact()?
                .to_string()
        }
    };

    let frameworks = get_framework(&registry, &project_lang);

    let project_framework = match def_framework {
        Some(fw) if frameworks.iter().any(|f| f == &fw) => {
            log::step(format!("Framework:\n{}", fw.bright_black()))?;
            fw
        }
        _ => {
            select("Select a framework:")
                .items(&to_select_items(&frameworks))
                .interact()?
                .to_string()
        }
    };

    let templates = get_templates(&registry, &project_lang, &project_framework);

    let project_template = match def_templt {
        Some(tpl) if check_template(&registry, &project_lang, &project_framework, &tpl) => {
            log::step(format!("Template:\n{}", tpl.bright_black()))?;
            tpl
        }
        _ => {
            select("Select a template:").items(&to_select_items(&templates)).interact()?.to_string()
        }
    };

    let spin = spinner();
    spin.start(format!("Scaffolding \"{}\" using \"{}\"", project_name, project_template));

    match scaffold_project(&project_lang, &project_framework, &project_template, &project_name) {
        Ok(_) => {
            spin.stop("Scaffolding completed.");
            note("Next steps:", format!("cd {}", project_name).as_str())?;
        }
        Err(e) => {
            spin.error(format!("Error: {}", e));
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
