#[derive(Debug)]
pub struct Item {
    pub id: &'static str,
    pub lang: &'static str,
    pub setup: &'static str,
    pub templates: &'static [&'static str],
}

static DATA: &[Item] = &[
    Item {
        id: "rust",
        lang: "Rust",
        setup: "cargo update\ncargo run",
        templates: &["actix-blank", "axum-blank", "rocket-blank", "warp-blank"],
    },
];

pub fn get_lang() -> Vec<(&'static str, &'static str, &'static str)> {
    DATA.iter()
        .map(|item| (item.id, item.lang, ""))
        .collect()
}

pub fn get_template(id: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    DATA.iter()
        .find(|item| item.id == id)
        .map(|item| {
            item.templates
                .iter()
                .map(|&t| (t, t, ""))
                .collect()
        })
        .unwrap_or_default()
}

pub fn check_lang(target: &str) -> bool {
    DATA.iter().any(|item| item.id == target)
}

pub fn check_template(target: &str, lang_id: &str) -> bool {
    DATA.iter()
        .find(|item| item.id == lang_id)
        .map(|item| item.templates.contains(&target))
        .unwrap_or(false)
}

pub fn get_setup_lang(id: &str) -> &'static str {
    DATA.iter()
        .find(|item| item.id == id)
        .map(|item| item.setup)
        .unwrap_or("")
}
