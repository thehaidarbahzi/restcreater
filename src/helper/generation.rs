use include_dir::{ include_dir, Dir };
use std::{ fs, path::Path, collections::HashMap };

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

pub fn scaffold_project(
    lang: &str,
    framework: &str,
    template: &str,
    name: &str
) -> anyhow::Result<()> {
    let dir = TEMPLATES.get_dir(&format!("{}/{}/{}", lang, framework, template)).ok_or_else(||
        anyhow::anyhow!("Template not found")
    )?;

    let mut vars = HashMap::new();
    vars.insert("name", name);
    vars.insert("lang", lang);

    extract(dir, Path::new(name), &vars)?;
    Ok(())
}

fn extract(dir: &Dir, target: &Path, vars: &HashMap<&str, &str>) -> std::io::Result<()> {
    fs::create_dir_all(target)?;

    for file in dir.files() {
        let path = target.join(file.path().file_name().unwrap());
        let content = std::str::from_utf8(file.contents()).unwrap();
        fs::write(path, replace(content, vars))?;
    }

    for sub in dir.dirs() {
        extract(sub, &target.join(sub.path().file_name().unwrap()), vars)?;
    }

    Ok(())
}

fn replace(content: &str, vars: &HashMap<&str, &str>) -> String {
    vars.iter().fold(content.to_string(), |acc, (k, v)| {
        acc.replace(&format!("{{{{{}}}}}", k), v)
    })
}
