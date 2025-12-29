use std::path::Path;
use std::fs;
use std::io::{ self, Write };
use walkdir::WalkDir;

pub fn ensure_unique_name(base_name: &str) -> String {
    let mut name = base_name.to_string();
    let mut counter = 0;

    while Path::new(&name).exists() {
        counter += 1;
        name = format!("{}-{}", base_name, counter);
    }

    name
}

pub fn check_folder_exists(name: &str) -> bool {
    Path::new(name).exists()
}

pub fn copy_template_to_project(
    template_path: &str,
    project_name: &str,
    project_config: &crate::helper::prompts::ProjectConfig
) -> io::Result<()> {
    let template_dir = Path::new(template_path);
    let project_dir = Path::new(project_name);

    fs::create_dir_all(project_dir)?;

    for entry in WalkDir::new(template_dir)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let template_file_path = entry.path();
        let relative_path = template_file_path.strip_prefix(template_dir).unwrap();
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

            let processed_content = replace_placeholders(&content, project_config);
            let mut file = fs::File::create(&project_file_path)?;
            file.write_all(processed_content.as_bytes())?;
        }
    }

    Ok(())
}

fn replace_placeholders(content: &str, config: &crate::helper::prompts::ProjectConfig) -> String {
    let lib_name = config.name.replace("-", "_");

    content.replace("{name}", &config.name).replace("{lib_name}", &lib_name)
}
