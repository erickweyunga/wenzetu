//! Configuration management
//!
//! Simplified config loading from environment variables with dotenv support.

use config::{Config, Environment};
use serde::Deserialize;
use uncovr::config::Environment as UncovREnvironment;

/// Application configuration
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AppConfig {
    /// Application configuration
    pub app: App,
    /// Project environment configuration
    pub environment: UncovREnvironment,
    /// Templates configuration
    pub templates: Templates,
    /// API documentation configuration
    pub docs: Docs,
}

/// Application settings
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct App {
    /// Application name
    pub name: String,
    /// Application description
    pub description: String,
    /// Application version
    pub version: String,
    /// App Public Address
    pub address: String,
    /// App Port
    pub port: u16,
}

/// Templates configuration
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Templates {
    /// Template directory path
    pub path: String,
    /// Static files directory
    pub static_dir: String,
    /// Static files serve path
    pub static_path: String,
}

/// Documentation configuration
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Docs {
    /// Swagger UI path
    pub docs_path: String,
    /// OpenAPI JSON path
    pub openapi_json_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: App::default(),
            environment: UncovREnvironment::default(),
            templates: Templates::default(),
            docs: Docs::default(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            address: "127.0.0.1".to_string(),
            port: 8000,
        }
    }
}

impl Default for Templates {
    fn default() -> Self {
        Self {
            path: "templates/**/*".to_string(),
            static_dir: "./public".to_string(),
            static_path: "/public".to_string(),
        }
    }
}

impl Default for Docs {
    fn default() -> Self {
        Self {
            docs_path: "/swagger".to_string(),
            openapi_json_path: "/openapi.json".to_string(),
        }
    }
}

/// Load configuration from environment variables.
///
/// Automatically loads .env file if present.
///
/// # Example
/// ```rust
/// use wenzetu::config::load_config;
///
/// let config = load_config();
/// println!("Server: {}:{}", config.app.address, config.app.port);
/// ```
pub fn load_config() -> AppConfig {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();

    let source = Environment::default().separator(".");

    let config = Config::builder().add_source(source).build();

    match config {
        Ok(cfg) => cfg
            .try_deserialize::<AppConfig>()
            .unwrap_or_else(|_| AppConfig::default()),
        Err(_) => AppConfig::default(),
    }
}

/// Create uncovr AppConfig from loaded configuration.
///
/// # Example
/// ```rust
/// use wenzetu::config::{load_config, to_uncovr_config};
///
/// let config = load_config();
/// let server_config = to_uncovr_config(&config);
/// ```
pub fn to_uncovr_config(config: &AppConfig) -> uncovr::config::AppConfig {
    let addr = format!("{}:{}", config.app.address, config.app.port);

    uncovr::config::AppConfig::new(config.app.name.clone(), config.app.version.clone())
        .description(config.app.description.clone())
        .environment(config.environment.clone())
        .bind(addr)
}
