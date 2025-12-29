use inquire::{ InquireError, Select, Text };
use inline_colorization::*;
use crate::helper::validation::{ is_valid_framework, is_valid_lang, is_valid_template };
use crate::helper::utils::check_folder_exists;

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub lang: String,
    pub framework: String,
    pub template: String,
}

pub fn smart_prompts(
    name: Option<String>,
    lang: Option<String>,
    framework: Option<String>,
    template: Option<String>
) -> Result<ProjectConfig, InquireError> {
    let name = match name {
        Some(n) if !n.trim().is_empty() => {
            if check_folder_exists(n.trim()) {
                println!(
                    "{color_red}✗ Error: Folder '{}' already exists. Please choose a different name.{color_reset}",
                    n.trim()
                );
                std::process::exit(1);
            }
            println!(
                "{color_bright_black}✓ Project name:{color_reset} {color_cyan}{}{color_reset}",
                n.trim()
            );
            n.trim().to_string()
        }
        Some(_) => {
            let unique_name = "my-api";
            if check_folder_exists(unique_name) {
                println!("{color_red}✗ Error: Folder '{}' already exists. Please choose a different name.{color_reset}", unique_name);
                std::process::exit(1);
            }
            println!("{color_bright_black}✓ Project name:{color_reset} {color_cyan}{}{color_reset}", unique_name);
            unique_name.to_string()
        }
        None => {
            loop {
                let input_name = Text::new("Project name:")
                    .with_placeholder("my-project")
                    .with_default("my-project")
                    .with_help_message("Enter your project name")
                    .prompt()?;

                let final_name = if input_name.trim().is_empty() {
                    "my-project".to_string()
                } else {
                    input_name.trim().to_string()
                };

                if check_folder_exists(&final_name) {
                    println!("{color_red}✗ Error: Folder '{}' already exists. Please choose a different name.{color_reset}", final_name);
                    continue;
                }

                break final_name;
            }
        }
    };

    let lang = match lang {
        Some(l) if is_valid_lang(&l) => {
            println!("{color_bright_black}✓ Language:{color_reset} {color_cyan}{}{color_reset}", l);
            l
        }
        _ => prompt_language()?,
    };

    let framework = match framework {
        Some(f) if is_valid_framework(&lang, &f) => {
            println!("{color_bright_black}✓ Framework:{color_reset} {color_cyan}{}{color_reset}", f);
            f
        }
        _ => prompt_framework(&lang)?,
    };

    let template = match template {
        Some(t) if is_valid_template(&t) => {
            println!("{color_bright_black}✓ Template:{color_reset} {color_cyan}{}{color_reset}", t);
            t
        }
        _ => prompt_template()?,
    };

    println!();
    println!("{color_bright_black}─────────────────────────────────{color_reset}");
    println!("{style_bold}Project Summary:{style_reset}");
    println!("   Name:      {color_cyan}{}{color_reset}", name);
    println!("   Language:  {color_cyan}{}{color_reset}", lang);
    println!("   Framework: {color_cyan}{}{color_reset}", framework);
    println!("   Template:  {color_cyan}{}{color_reset}", template);
    println!("{color_bright_black}─────────────────────────────────{color_reset}");
    println!();

    Ok(ProjectConfig {
        name,
        lang,
        framework,
        template,
    })
}

fn prompt_language() -> Result<String, InquireError> {
    let languages = vec!["Rust"];
    let choice = Select::new("Select a language:", languages)
        .with_help_message("↑↓ to move, enter to select")
        .prompt()?;

    Ok(
        (
            match choice {
                "Rust" => "rust",
                _ => "rust",
            }
        ).to_string()
    )
}

fn prompt_framework(lang: &str) -> Result<String, InquireError> {
    let frameworks = get_frameworks(lang);
    let choice = Select::new("Select a framework:", frameworks)
        .with_help_message("↑↓ to move, enter to select")
        .prompt()?;

    Ok(extract_framework_name(choice))
}
//TODO: Add more templates
// "Modular - Project structure with modular setup",
// "JWT - Project structure with JWT authentication",
// "ORM - Project structure with ORM integration",
// "Docker - Project structure with Docker setup",
// "Full - Project structure with all features included"

//TODO: Add more template choices
// s if s.contains("Modular") => "modular",
// s if s.contains("JWT") => "jwt",
// s if s.contains("ORM") => "orm",
// s if s.contains("Docker") => "docker",
// s if s.contains("Full") => "full",

fn prompt_template() -> Result<String, InquireError> {
    let templates = vec!["Blank - Minimal project structure"];
    let choice = Select::new("Select a template:", templates)
        .with_help_message("↑↓ to move, enter to select")
        .prompt()?;

    Ok(
        (
            match choice {
                s if s.contains("Blank") => "blank",
                _ => "blank",
            }
        ).to_string()
    )
}

fn get_frameworks(lang: &str) -> Vec<&'static str> {
    match lang {
        "rust" => vec!["Axum", "Actix", "Rocket", "Warp"],
        _ => vec!["None"],
    }
}

fn extract_framework_name(choice: &str) -> String {
    choice.split_whitespace().next().unwrap_or("unknown").to_lowercase()
}
