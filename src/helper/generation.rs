use inline_colorization::*;
use crate::helper::prompts::ProjectConfig;
use crate::helper::utils::copy_template_to_project;
use std::path::Path;

pub fn generate_project(config: &ProjectConfig) {
    println!("{color_green}Creating project...{color_reset}");

    let template_path = get_template_path(&config.lang, &config.framework, &config.template);

    if !Path::new(&template_path).exists() {
        println!("{color_red}✗ Error: Template not found at {}{color_reset}", template_path);
        std::process::exit(1);
    }

    println!("{color_bright_black}📂 Using template: {}{color_reset}", template_path);

    match copy_template_to_project(&template_path, &config.name, config) {
        Ok(_) => {
            println!();
            println!(
                "{color_green}{style_bold}✓ Project '{}' created successfully!{style_reset}{color_reset}",
                config.name
            );
        }
        Err(e) => {
            println!("{color_red}✗ Error creating project: {}{color_reset}", e);
            std::process::exit(1);
        }
    }

    println!();
    println!("Next steps:");
    println!("  {color_cyan}cd {}{color_reset}", config.name);

    match config.lang.as_str() {
        "rust" => {
            println!("  {color_cyan}cargo update{color_reset}");
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

fn get_template_path(lang: &str, framework: &str, template: &str) -> String {
    // Get the executable path and construct template path relative to it
    let exe_path = std::env::current_exe().unwrap_or_else(|_| {
        // Fallback to current dir if can't get exe path (for development)
        std::env::current_dir().unwrap().join("target/debug/restcreater")
    });

    let exe_dir = exe_path.parent().unwrap_or_else(|| {
        eprintln!("{color_red}✗ Error: Failed to determine executable directory.{color_reset}");
        std::process::exit(1);
    });

    // Try multiple possible locations for templates
    let possible_paths = vec![
        // When running from project root (development)
        format!("src/templates/{}/{}-{}", lang, framework, template),
        // When running from target/debug or target/release
        format!("../../../src/templates/{}/{}-{}", lang, framework, template),
        // Installed location (future use)
        format!("templates/{}/{}-{}", lang, framework, template)
    ];

    for path in possible_paths {
        let full_path = exe_dir.join(&path);
        if full_path.exists() {
            return full_path.to_string_lossy().to_string();
        }
    }

    // Default fallback
    format!("src/templates/{}/{}-{}", lang, framework, template)
}
