use std::path::Path;
use std::fs;
use std::io::{ self, Write };
use walkdir::WalkDir;

pub fn sanitize_name(name: &str) -> String {
    let sanitized = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() { c } else if c.is_whitespace() || c == '_' { '-' } else { '-' }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-");

    let mut unique_name = sanitized.clone();
    let mut counter = 0;
    while Path::new(&unique_name).exists() {
        counter += 1;
        unique_name = format!("{}-{}", sanitized, counter);
    }
    unique_name
}

pub fn check_folder_exists(name: &str) -> bool {
    Path::new(name).exists()
}

pub fn copy_template_to_project(template_path: &str, project_name: &str) -> io::Result<()> {
    let template_dir = Path::new(template_path);
    let project_dir = Path::new(project_name);

    fs::create_dir_all(project_dir)?;

    for entry in WalkDir::new(template_dir)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let template_file_path = entry.path();
        let relative_path = template_file_path
            .strip_prefix(template_dir)
            .expect("template_file_path is not within template_dir when copying template");
        let project_file_path = project_dir.join(relative_path);

        if template_file_path.is_dir() {
            fs::create_dir_all(&project_file_path)?;
        } else {
            if let Some(parent) = project_file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = match fs::read_to_string(template_file_path) {
                Ok(content) => content,
                Err(_) => {
                    fs::copy(template_file_path, &project_file_path)?;
                    continue;
                }
            };

            let processed_content = replace_placeholders(&content, project_name);
            let mut file = fs::File::create(&project_file_path)?;
            file.write_all(processed_content.as_bytes())?;
        }
    }

    Ok(())
}

fn replace_placeholders(content: &str, new_name: &str) -> String {
    content.replace("{name}", new_name)
}

pub fn capitalize_text(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
