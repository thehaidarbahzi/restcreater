pub fn is_valid_lang(lang: &str) -> bool {
    matches!(lang.to_lowercase().as_str(), "rust" | "go" | "node" | "python")
}

pub fn is_valid_framework(lang: &str, framework: &str) -> bool {
    match lang.to_lowercase().as_str() {
        "rust" => ["axum", "actix", "rocket", "warp"].contains(&framework.to_lowercase().as_str()),
        "go" => ["gin", "fiber", "echo", "gorilla"].contains(&framework.to_lowercase().as_str()),
        "node" =>
            ["express", "fastify", "nestjs", "koa"].contains(&framework.to_lowercase().as_str()),
        "python" =>
            ["fastapi", "flask", "django", "starlette"].contains(
                &framework.to_lowercase().as_str()
            ),
        _ => false,
    }
}

pub fn is_valid_template(template: &str) -> bool {
    matches!(template.to_lowercase().as_str(), "blank" | "mvc" | "auth" | "orm" | "docker" | "full")
}
