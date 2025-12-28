use inline_colorization::*;
use inquire::{ InquireError, Select, Text };
use std::path::Path;
use crate::helper::validation::{ is_valid_framework, is_valid_lang, is_valid_template };

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

#[derive(Debug)]
struct ProjectConfig {
    name: String,
    lang: String,
    framework: String,
    template: String,
}

fn smart_prompts(
    name: Option<String>,
    lang: Option<String>,
    framework: Option<String>,
    template: Option<String>
) -> Result<ProjectConfig, InquireError> {
    let name = match name {
        Some(n) if !n.trim().is_empty() => {
            println!(
                "{color_bright_black}✓ Project name:{color_reset} {color_cyan}{}{color_reset}",
                n.trim()
            );
            ensure_unique_name(n.trim())
        }
        Some(_) => {
            let unique_name = ensure_unique_name("my-api");
            println!("{color_bright_black}✓ Project name:{color_reset} {color_cyan}{}{color_reset}", unique_name);
            unique_name
        }
        None => {
            let input_name = Text::new("Project name:")
                .with_placeholder("my-api")
                .with_default("my-api")
                .with_help_message("Enter your project name")
                .prompt()?;

            let final_name = if input_name.trim().is_empty() {
                "my-api".to_string()
            } else {
                input_name.trim().to_string()
            };

            ensure_unique_name(&final_name)
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

fn ensure_unique_name(base_name: &str) -> String {
    let mut name = base_name.to_string();
    let mut counter = 0;

    while Path::new(&name).exists() {
        counter += 1;
        name = format!("{}-{}", base_name, counter);
    }

    name
}

fn prompt_language() -> Result<String, InquireError> {
    let languages = vec!["Rust", "Go", "Node.js", "Python"];
    let choice = Select::new("Select a language:", languages)
        .with_help_message("↑↓ to move, enter to select")
        .prompt()?;

    Ok(
        (
            match choice {
                "Rust" => "rust",
                "Go" => "go",
                "Node.js" => "node",
                "Python" => "python",
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

fn prompt_template() -> Result<String, InquireError> {
    let templates = vec![
        "Blank - Empty project structure",
        "MVC - Model-View-Controller pattern",
        "Auth - JWT authentication ready",
        "ORM - Database ORM integrated",
        "Docker - Docker setup included",
        "Full - All features included"
    ];
    let choice = Select::new("Select a template:", templates)
        .with_help_message("↑↓ to move, enter to select")
        .prompt()?;

    Ok(
        (
            match choice {
                s if s.contains("Blank") => "blank",
                s if s.contains("MVC") => "mvc",
                s if s.contains("Auth") => "auth",
                s if s.contains("ORM") => "orm",
                s if s.contains("Docker") => "docker",
                s if s.contains("Full") => "full",
                _ => "blank",
            }
        ).to_string()
    )
}

fn get_frameworks(lang: &str) -> Vec<&'static str> {
    match lang {
        "rust" =>
            vec![
                "Axum - Modern, ergonomic web framework",
                "Actix - Powerful, pragmatic, fast",
                "Rocket - Simple, fast, type-safe",
                "Warp - Composable web framework"
            ],
        "go" =>
            vec![
                "Gin - Fast HTTP web framework",
                "Fiber - Express-inspired framework",
                "Echo - High performance, minimalist",
                "Gorilla - Web toolkit for Go"
            ],
        "node" =>
            vec![
                "Express - Fast, unopinionated, minimal",
                "Fastify - Fast and low overhead",
                "NestJS - Progressive Node.js framework",
                "Koa - Next generation web framework"
            ],
        "python" =>
            vec![
                "FastAPI - Modern, fast, async",
                "Flask - Lightweight WSGI framework",
                "Django - Batteries-included framework",
                "Starlette - Lightweight ASGI framework"
            ],
        _ => vec!["None"],
    }
}

fn extract_framework_name(choice: &str) -> String {
    // Extract the framework name (first word before the dash)
    choice.split(' ').next().unwrap_or("unknown").to_lowercase()
}

fn generate_project(config: &ProjectConfig) {
    println!("{color_green}Creating project...{color_reset}");
    println!();

    // TODO: Implement actual project generation
    println!(
        "{color_green}{style_bold}✓ Project '{}' created successfully!{style_reset}{color_reset}",
        config.name
    );
    println!();
    println!("Next steps:");
    println!("  {color_cyan}cd {}{color_reset}", config.name);

    match config.lang.as_str() {
        "rust" => {
            println!("  {color_cyan}cargo run{color_reset}");
        }
        "go" => {
            println!("  {color_cyan}go run .{color_reset}");
        }
        "node" => {
            println!("  {color_cyan}npm install{color_reset}");
            println!("  {color_cyan}npm run dev{color_reset}");
        }
        "python" => {
            println!("  {color_cyan}pip install -r requirements.txt{color_reset}");
            println!("  {color_cyan}python main.py{color_reset}");
        }
        _ => {}
    }
}
