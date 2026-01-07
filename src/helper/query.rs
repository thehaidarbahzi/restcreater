use super::registry::{ Item };

pub fn check_lang(registry: &[Item], lang: &str) -> bool {
    registry.iter().any(|l| l.id == lang)
}

pub fn get_lang(registry: &[Item]) -> Vec<String> {
    registry
        .iter()
        .map(|l| l.id.clone())
        .collect()
}

pub fn get_framework(registry: &[Item], lang: &str) -> Vec<String> {
    registry
        .iter()
        .find(|l| l.id == lang)
        .map(|l|
            l.frameworks
                .iter()
                .map(|f| f.id.clone())
                .collect()
        )
        .unwrap_or_default()
}

pub fn get_templates(registry: &[Item], lang: &str, framework: &str) -> Vec<String> {
    registry
        .iter()
        .find(|l| l.id == lang)
        .and_then(|l| l.frameworks.iter().find(|f| f.id == framework))
        .map(|f| f.templates.clone())
        .unwrap_or_default()
}

pub fn check_template(registry: &[Item], lang: &str, framework: &str, template: &str) -> bool {
    get_templates(registry, lang, framework)
        .iter()
        .any(|t| t == template)
}
