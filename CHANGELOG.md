# Changelog

All notable changes to Wenzetu will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1]

### Added

#### Configuration System
- **Environment configuration**: Environment (Development/Staging/Production) is now fully configurable
- **Environment variables support**: All settings can be configured via `.env` file
- **Templates path configuration**: Customize template directory via `TEMPLATES.PATH` or builder API
- **Static files configuration**: Configure static directory and serve path
- **API docs path configuration**: Customize Swagger UI and OpenAPI JSON paths
- **Builder API methods**:
  - `.environment()` - Set application environment
  - `.templates_path()` - Set custom templates directory
  - `.docs_path()` - Set Swagger UI path
  - `.openapi_json_path()` - Set OpenAPI JSON path
  - `.static_files()` - Configure static file serving
  - `.no_static_files()` - Disable static file serving
  - `.api_no_docs()` - Add API routes without documentation
  - `.live_reload()` - Manually control live reload

#### Configuration Structs
- `Templates` struct in `AppConfig` with fields:
  - `path`: Template directory path (default: "templates/**/*")
  - `static_dir`: Static files directory (default: "./public")
  - `static_path`: Static files serve path (default: "/public")
- `Docs` struct in `AppConfig` with fields:
  - `docs_path`: Swagger UI path (default: "/swagger")
  - `openapi_json_path`: OpenAPI JSON path (default: "/openapi.json")

#### Helper Functions
- `helpers::fullstack_configs_custom()` - Create fullstack configs with custom docs paths
- `templates::init_templates()` - Initialize templates with custom path
- `templates::get_template_path()` - Get configured template path

#### Documentation
- `CONFIGURATION.md` - Comprehensive configuration guide with examples
- `MIGRATION.md` - Step-by-step migration guide for upgrading
- `examples/custom_config.rs` - Complete working examples of all configuration options

### Changed

#### Breaking Changes
- **`helpers::web_config()`** now requires `environment: Environment` parameter
  - Before: `web_config(name, version)`
  - After: `web_config(name, version, environment)`

- **`helpers::api_config()`** now requires additional parameters:
  - Before: `api_config(name, version, addr)`
  - After: `api_config(name, version, addr, environment, docs_path, openapi_json_path)`

#### Non-Breaking Changes
- `helpers::fullstack_configs()` - Still works with defaults, no changes needed
- `App::new()` - Enhanced with new configuration methods
- `templates::TEMPLATES` - Now respects configured template path
- Template hot-reload - Now watches configured template directory
- `auto_config()` - Now loads template paths, static paths, and docs paths from environment

### Improved

- **Flexibility**: Every setting can now be customized
- **Environment support**: Proper Development/Staging/Production modes
- **Configuration priority**: Clear precedence (Builder API > Environment Variables > Defaults)
- **Developer experience**: Fluent, chainable builder API
- **Production readiness**: Environment-specific configurations
- **Testing**: Easy to override settings for different test scenarios
- **DevOps friendly**: Configuration via environment variables
- **Documentation**: Comprehensive guides and examples

### Fixed

- Templates now use configured path instead of hardcoded "templates/**/*"
- Static files now use configured paths instead of hardcoded "./public" and "/public"
- API documentation paths now configurable instead of hardcoded
- Environment no longer hardcoded to Development in `web_config()`
- Live reload watcher now uses configured template path

## Migration Guide

See [MIGRATION.md](./MIGRATION.md) for detailed upgrade instructions.

### Quick Migration

If you're using `fullstack_configs()`, no changes needed! Otherwise:

```rust
// Before
let web = helpers::web_config("App", "1.0.0");

// After - add environment parameter
use uncovr::config::Environment;
let web = helpers::web_config("App", "1.0.0", Environment::Development);
```

```rust
// Before
let api = helpers::api_config("API", "1.0.0", "127.0.0.1:8000");

// After - add environment and docs paths
let api = helpers::api_config(
    "API",
    "1.0.0",
    "127.0.0.1:8000",
    Environment::Production,
    "/swagger",
    "/openapi.json"
);
```

## Configuration Examples

### Via Environment Variables

```bash
# .env
ENVIRONMENT=Production
TEMPLATES.PATH=views/**/*
TEMPLATES.STATIC_DIR=./static
TEMPLATES.STATIC_PATH=/assets
DOCS.DOCS_PATH=/documentation
DOCS.OPENAPI_JSON_PATH=/api-spec.json
```

### Via Builder API

```rust
App::new()
    .auto_config()
    .environment(Environment::Production)
    .templates_path("views/**/*")
    .static_files("/assets", "./static")
    .docs_path("/documentation")
    .openapi_json_path("/api-spec.json")
    .web(web_routes)
    .api("/api", api_routes)
    .serve()
    .await
    .unwrap();
```

## Benefits

✅ **Full Customization** - Configure every aspect of your application
✅ **Environment Support** - Proper Dev/Staging/Prod configurations
✅ **DevOps Friendly** - Configure via environment variables
✅ **Production Ready** - Environment-specific settings
✅ **Developer Friendly** - Fluent builder API
✅ **Backward Compatible** - `fullstack_configs()` still works
✅ **Well Documented** - Comprehensive guides and examples

## Resources

- [CONFIGURATION.md](./CONFIGURATION.md) - Configuration guide
- [MIGRATION.md](./MIGRATION.md) - Migration instructions
- [README.md](./README.md) - Updated API reference
- [examples/custom_config.rs](./examples/custom_config.rs) - Working examples

---

**Note**: This is a major enhancement that makes Wenzetu fully configurable while maintaining backward compatibility for the most common use case (`fullstack_configs()`).
