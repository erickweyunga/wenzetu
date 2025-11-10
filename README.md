# WENZETU

**Wenzetu** is a highly configurable helper crate for [Uncovr](https://github.com/erickweyunga/uncovr) projects that simplifies common web application setup tasks.

## What it does

Wenzetu extracts boilerplate code so you can focus on building your application:

- ✅ **Template Rendering** - Tera integration with hot-reload in development
- ✅ **Static File Serving** - Configurable asset serving
- ✅ **Configuration Management** - Flexible config loading from environment variables
- ✅ **Live Reload** - Browser auto-refresh in development mode
- ✅ **Builder API** - Fluent, chainable configuration
- ✅ **Full Customization** - Override any default setting

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wenzetu = "0.1"
uncovr = "0.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

Or install via cargo:

```bash
cargo add wenzetu uncovr
```

## Quick Start

### Simple Full-Stack App

```rust
use wenzetu::prelude::*;

#[derive(Clone)]
struct AppState;

#[tokio::main]
async fn main() {
    let config = load_config();
    let (web_config, api_config) = helpers::fullstack_configs(&config);

    let web_routes = app::routes::create_routes(AppState, web_config).await;
    let api_routes = api::routes::create_api_routes(AppState, api_config).await;

    // Simple setup with sensible defaults
    App::new()
        .auto_config()
        .web(web_routes)
        .api("/api", api_routes)
        .serve()
        .await
        .unwrap();
}
```

That's it! This gives you:
- Web routes with template rendering
- API routes with OpenAPI documentation at `/swagger`
- Static file serving from `./public` at `/public`
- Live reload in development
- Configuration from `.env` file

## Configuration

### Three Ways to Configure

Wenzetu supports flexible configuration with clear priority:

1. **Builder API** (highest priority) - Method calls like `.docs_path("/custom")`
2. **Environment Variables** - Via `.env` file or system environment
3. **Defaults** (lowest priority) - Sensible defaults

### Environment Variables

Create a `.env` file:

```bash
# Application Settings
APP.NAME=my-blog
APP.DESCRIPTION=My awesome blog
APP.VERSION=1.0.0
APP.ADDRESS=127.0.0.1
APP.PORT=8000

# Environment (Development, Staging, Production)
ENVIRONMENT=Development

# Templates Configuration
TEMPLATES.PATH=templates/**/*
TEMPLATES.STATIC_DIR=./public
TEMPLATES.STATIC_PATH=/public

# API Documentation
DOCS.DOCS_PATH=/swagger
DOCS.OPENAPI_JSON_PATH=/openapi.json
```

### Builder API Configuration

Override any setting programmatically:

```rust
App::new()
    .auto_config()  // Load from environment first

    // Then override what you need
    .environment(uncovr::config::Environment::Production)
    .templates_path("custom/templates/**/*")
    .docs_path("/api-docs")
    .openapi_json_path("/api-spec.json")
    .static_files("/assets", "./static")

    .web(web_routes)
    .api("/api", api_routes)
    .serve()
    .await
    .unwrap();
```

## Features

### Builder Methods

```rust
App::new()
    // Configuration
    .auto_config()                          // Load from .env
    .with_config(custom_config)             // Use custom config
    .environment(Environment::Production)   // Set environment

    // Templates
    .templates_path("views/**/*")           // Custom template path

    // Static Files
    .static_files("/assets", "./public")    // Custom static files
    .no_static_files()                      // Disable static files

    // API Documentation
    .docs_path("/documentation")            // Custom Swagger UI path
    .openapi_json_path("/spec.json")        // Custom OpenAPI JSON path

    // Routes
    .web(web_routes)                        // Add web routes
    .api("/api", api_routes)                // Add API with docs
    .api_no_docs("/api", api_routes)        // Add API without docs

    // Development
    .live_reload(true)                      // Enable/disable live reload

    .serve()                                // Build and serve
    .await
    .unwrap();
```

### Helper Functions

```rust
use wenzetu::prelude::*;
use uncovr::config::Environment;

// Configurable web config
let web = helpers::web_config(
    "My App",
    "1.0.0",
    Environment::Production  // Now configurable!
);

// Configurable API config
let api = helpers::api_config(
    "My API",
    "1.0.0",
    "127.0.0.1:8000",
    Environment::Production,
    "/documentation",     // Custom docs path
    "/spec.json"         // Custom OpenAPI path
);

// Fullstack with custom docs paths
let config = load_config();
let (web, api) = helpers::fullstack_configs_custom(
    &config,
    "/api-docs",
    "/openapi-spec.json"
);
```

## Examples

### Example 1: Production Setup

```rust
App::new()
    .auto_config()
    .environment(uncovr::config::Environment::Production)
    .live_reload(false)
    .web(web_routes)
    .api("/api", api_routes)
    .serve()
    .await
    .unwrap();
```

### Example 2: Custom Paths

```rust
App::new()
    .auto_config()
    .templates_path("views/**/*")
    .static_files("/static", "./assets")
    .docs_path("/documentation")
    .openapi_json_path("/api-schema.json")
    .web(web_routes)
    .api("/v1", api_routes)
    .serve()
    .await
    .unwrap();
```

### Example 3: API Only (No Web UI)

```rust
App::new()
    .auto_config()
    .no_static_files()
    .docs_path("/docs")
    .api("/api", api_routes)
    .serve()
    .await
    .unwrap();
```

### Example 4: Multiple Static Directories

```rust
use wenzetu::static_files;

App::new()
    .web(web_routes)
    .merge(static_files::serve_dir("/css", "./styles"))
    .merge(static_files::serve_dir("/js", "./scripts"))
    .merge(static_files::serve_dir("/images", "./media"))
    .serve()
    .await
    .unwrap();
```

### Example 5: Staging Environment

**.env.staging**:
```bash
ENVIRONMENT=Staging
APP.ADDRESS=0.0.0.0
APP.PORT=3000
DOCS.DOCS_PATH=/internal-docs
```

```rust
// Load staging config
App::new()
    .auto_config()  // Loads ENVIRONMENT=Staging from .env
    .web(web_routes)
    .api("/api", api_routes)
    .serve()
    .await
    .unwrap();
```

## Template Rendering

### Setup Templates

Create templates in `templates/` directory (or your custom path):

**templates/base.html**:
```html
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}My App{% endblock %}</title>
    <link rel="stylesheet" href="/public/style.css">
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```

**templates/index.html**:
```html
{% extends "base.html" %}
{% block title %}{{ title }}{% endblock %}
{% block content %}
    <h1>{{ message }}</h1>
{% endblock %}
```

### Use in Your Handler

```rust
use wenzetu::{render, context};
use uncovr::{prelude::*, response::Html};

#[derive(Clone)]
struct HomePage;

impl Endpoint for HomePage {
    fn ep(&self) -> Route {
        Route::GET("/")
    }
}

#[async_trait]
impl API for HomePage {
    type Req = ();
    type Res = Html<String>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let html = render("index.html", &context! {
            title: "Welcome",
            message: "Hello from Wenzetu!",
        });
        Html(html)
    }
}
```

### Custom Template Path

```rust
// Via builder
App::new()
    .templates_path("views/**/*")
    .web(web_routes)
    .serve()
    .await
    .unwrap();

// Via environment
// TEMPLATES.PATH=views/**/*
```

## Static Files

### Default Behavior

By default, files in `./public` are served at `/public`:

```
./public/style.css  → http://localhost:8000/public/style.css
./public/app.js     → http://localhost:8000/public/app.js
```

### Custom Static Files

```rust
App::new()
    .static_files("/assets", "./static")
    // ./static/style.css → http://localhost:8000/assets/style.css
```

### Via Environment

```bash
TEMPLATES.STATIC_DIR=./static
TEMPLATES.STATIC_PATH=/assets
```

## API Documentation

### Default Paths

- Swagger UI: `http://localhost:8000/swagger`
- OpenAPI JSON: `http://localhost:8000/openapi.json`

### Custom Paths

```rust
App::new()
    .auto_config()
    .docs_path("/documentation")
    .openapi_json_path("/api-spec.json")
    .api("/api", api_routes)
```

### Via Environment

```bash
DOCS.DOCS_PATH=/documentation
DOCS.OPENAPI_JSON_PATH=/api-spec.json
```

### Disable Documentation

```rust
App::new()
    .api_no_docs("/api", api_routes)
```

## Live Reload

Live reload is automatically enabled in debug builds and disabled in release builds.

### Manual Control

```rust
App::new()
    .auto_config()
    .live_reload(true)   // Force enable
    .web(web_routes)
    .serve()
    .await
    .unwrap();
```

### Disable in Development

```rust
App::new()
    .auto_config()
    .live_reload(false)  // Disable even in debug
    .web(web_routes)
    .serve()
    .await
    .unwrap();
```

## API Reference

### App Builder

```rust
impl App {
    pub fn new() -> Self
    pub fn with_config(self, config: UncovRConfig) -> Self
    pub fn auto_config(self) -> Self
    pub fn environment(self, env: Environment) -> Self
    pub fn templates_path(self, path: impl Into<String>) -> Self
    pub fn docs_path(self, path: impl Into<String>) -> Self
    pub fn openapi_json_path(self, path: impl Into<String>) -> Self
    pub fn web(self, routes: ApiRouter) -> Self
    pub fn api(self, path: impl Into<String>, routes: ApiRouter) -> Self
    pub fn api_no_docs(self, path: impl Into<String>, routes: ApiRouter) -> Self
    pub fn static_files(self, serve_path: impl Into<String>, directory: impl Into<String>) -> Self
    pub fn no_static_files(self) -> Self
    pub fn live_reload(self, enabled: bool) -> Self
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>>
}
```

### Template Functions

```rust
// Render a template
pub fn render(name: &str, context: &Context) -> String

// Initialize templates with custom path
pub fn init_templates(path: impl Into<String>)

// Create context with macro
context! {
    key1: value1,
    key2: value2,
}
```

### Static Files

```rust
// Serve directory at path
pub fn serve_dir(path: &str, directory: &str) -> ApiRouter

// Add default static routes
pub fn add_static_routes() -> ApiRouter
```

### Configuration

```rust
// Load from environment
pub fn load_config() -> AppConfig

// Convert to uncovr config
pub fn to_uncovr_config(config: &AppConfig) -> uncovr::config::AppConfig
```

### Helper Functions

```rust
// Web config with environment
pub fn web_config(
    name: impl Into<String>,
    version: impl Into<String>,
    environment: Environment,
) -> AppConfig

// API config with custom paths
pub fn api_config(
    name: impl Into<String>,
    version: impl Into<String>,
    addr: impl Into<String>,
    environment: Environment,
    docs_path: impl Into<String>,
    openapi_json_path: impl Into<String>,
) -> AppConfig

// Fullstack configs
pub fn fullstack_configs(cfg: &AppConfig) -> (AppConfig, AppConfig)

// Fullstack configs with custom docs paths
pub fn fullstack_configs_custom(
    cfg: &AppConfig,
    docs_path: impl Into<String>,
    openapi_json_path: impl Into<String>,
) -> (AppConfig, AppConfig)
```

## Cargo Features

### Default Features

- `live-reload` - Enables hot-reload for templates and live browser reload

### Disable Live Reload

```toml
[dependencies]
wenzetu = { path = "../wenzetu", default-features = false }
```

## Why Wenzetu?

**Before** (with boilerplate):
- 82 lines in `main.rs`
- 648 lines in `settings/service.rs`
- Manual Tera setup, error handling, static files, config loading
- Hardcoded paths and settings

**After** (with wenzetu):
- ~20-30 lines in `main.rs`
- Fully configurable via environment or code
- Focus on your application logic
- All boilerplate handled by wenzetu

## Complete Example

```rust
use wenzetu::prelude::*;

#[derive(Clone)]
pub struct AppState;

#[tokio::main]
async fn main() {
    // Load config
    let config = load_config();

    // Create route configs
    let (web_config, api_config) = helpers::fullstack_configs(&config);

    // Create routes
    let web_routes = app::routes::create_routes(AppState, web_config).await;
    let api_routes = api::routes::create_api_routes(AppState, api_config).await;

    // Build and serve with full control
    App::new()
        .auto_config()                              // Load from .env
        .environment(uncovr::config::Environment::Production)  // Override if needed
        .templates_path("templates/**/*")           // Custom templates
        .static_files("/public", "./public")        // Custom static files
        .docs_path("/swagger")                      // Custom docs path
        .openapi_json_path("/openapi.json")         // Custom OpenAPI path
        .live_reload(cfg!(debug_assertions))        // Dev only
        .web(web_routes)                            // Web routes
        .api("/api", api_routes)                    // API routes
        .serve()
        .await
        .unwrap();
}
```

## Documentation

For comprehensive configuration examples, see [CONFIGURATION.md](./CONFIGURATION.md)

## Repository

- **GitHub**: https://github.com/erickweyunga/wenzetu
- **Crates.io**: https://crates.io/crates/wenzetu
- **Documentation**: https://docs.rs/wenzetu

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

Eric Kweyunga - [GitHub](https://github.com/erickweyunga)
