use include_dir::{ include_dir, Dir };
use regex::Regex;
use std::collections::HashMap;
use std::{ fs };
use std::path::{ Path, PathBuf };

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

fn replace_placeholders(content: &str, vars: &HashMap<&str, &str>) -> String {
    let mut result = content.to_owned();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

fn extract_embedded_dir(
    embedded_dir: &Dir,
    target_path: &Path,
    vars: &HashMap<&str, &str>
) -> std::io::Result<()> {
    fs::create_dir_all(target_path)?;

    for file in embedded_dir.files() {
        let file_name = file
            .path()
            .file_name()
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file path"))?;

        let file_path = target_path.join(file_name);

        let contents = std::str
            ::from_utf8(file.contents())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let rendered = replace_placeholders(contents, vars);

        fs::write(&file_path, rendered)?;
    }

    for subdir in embedded_dir.dirs() {
        let subdir_name = subdir
            .path()
            .file_name()
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid directory path"))?;
        let subdir_path = target_path.join(subdir_name);
        extract_embedded_dir(subdir, &subdir_path, vars)?;
    }

    Ok(())
}

fn sanitize_name(name: &str) -> String {
    let re = Regex::new(r"[^\w\d_-]").unwrap();
    let sanitized = re.replace_all(name, "-");
    sanitized.to_string()
}

pub fn scaffold_project(
    lang: &str,
    template: &str,
    name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let template_path = format!("{}/{}", lang, template);
    let template_dir = TEMPLATES.get_dir(&template_path).ok_or("Template not found")?;

    let sanitized_name = sanitize_name(name);
    let target_dir = PathBuf::from(&sanitized_name);

    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("name", &sanitized_name);
    vars.insert("lang", lang);

    extract_embedded_dir(template_dir, &target_dir, &vars)?;

    Ok(())
}
