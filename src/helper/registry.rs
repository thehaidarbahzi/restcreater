use include_dir::{ include_dir, Dir };

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub setup: String,
    pub frameworks: Vec<FrameworkItem>,
}

#[derive(Debug, Clone)]
pub struct FrameworkItem {
    pub id: String,
    pub templates: Vec<String>,
}

pub fn scan_templates() -> Vec<Item> {
    TEMPLATES.dirs()
        .map(|lang_dir| {
            let lang_id = dir_name(lang_dir);

            Item {
                id: lang_id,
                setup: String::new(),
                frameworks: scan_frameworks(lang_dir),
            }
        })
        .collect()
}

fn scan_frameworks(lang_dir: &Dir) -> Vec<FrameworkItem> {
    lang_dir
        .dirs()
        .map(|fw_dir| FrameworkItem {
            id: dir_name(fw_dir),
            templates: fw_dir.dirs().map(dir_name).collect(),
        })
        .collect()
}

fn dir_name(dir: &Dir) -> String {
    dir.path().file_name().unwrap().to_string_lossy().to_string()
}
