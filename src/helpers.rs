//! Helper functions for common route setup patterns

use uncovr::config::{AppConfig, Environment};

/// Create a web app configuration with custom environment
pub fn web_config(
    name: impl Into<String>,
    version: impl Into<String>,
    environment: Environment,
) -> AppConfig {
    AppConfig::new(name.into(), version.into())
        .environment(environment)
        .docs(false)
}

/// Create an API configuration with OpenAPI docs
pub fn api_config(
    name: impl Into<String>,
    version: impl Into<String>,
    addr: impl Into<String>,
    environment: Environment,
    docs_path: impl Into<String>,
    openapi_json_path: impl Into<String>,
) -> AppConfig {
    let addr = addr.into();
    let docs_path = docs_path.into();
    let openapi_json_path = openapi_json_path.into();

    AppConfig::new(name.into(), version.into())
        .environment(environment)
        .docs_path(&docs_path)
        .openapi_json_path(&openapi_json_path)
        .bind(addr.clone())
        .add_server(format!("http://{}/api", addr), "API Server")
        .docs(true)
}

/// Create both web and API configs from loaded config
pub fn fullstack_configs(cfg: &crate::config::AppConfig) -> (AppConfig, AppConfig) {
    let addr = format!("{}:{}", cfg.app.address, cfg.app.port);

    let web = AppConfig::new(cfg.app.name.clone(), cfg.app.version.clone())
        .description(cfg.app.description.clone())
        .environment(cfg.environment.clone())
        .docs(false);

    let api = AppConfig::new(format!("{} API", cfg.app.name), cfg.app.version.clone())
        .description("DOCS")
        .docs_path("/swagger")
        .openapi_json_path("/api.json")
        .environment(cfg.environment.clone())
        .bind(addr.clone())
        .add_server(format!("http://{}/api", addr), "API Server")
        .docs(true);

    (web, api)
}

/// Create both web and API configs with custom paths
pub fn fullstack_configs_custom(
    cfg: &crate::config::AppConfig,
    docs_path: impl Into<String>,
    openapi_json_path: impl Into<String>,
) -> (AppConfig, AppConfig) {
    let addr = format!("{}:{}", cfg.app.address, cfg.app.port);
    let docs_path = docs_path.into();
    let openapi_json_path = openapi_json_path.into();

    let web = AppConfig::new(cfg.app.name.clone(), cfg.app.version.clone())
        .description(cfg.app.description.clone())
        .environment(cfg.environment.clone())
        .docs(false);

    let api = AppConfig::new(format!("{} API", cfg.app.name), cfg.app.version.clone())
        .description("DOCS")
        .docs_path(&docs_path)
        .openapi_json_path(&openapi_json_path)
        .environment(cfg.environment.clone())
        .bind(addr.clone())
        .add_server(format!("http://{}/api", addr), "API Server")
        .docs(true);

    (web, api)
}
