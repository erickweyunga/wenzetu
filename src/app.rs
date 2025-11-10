//! Application builder for simplified setup

use uncovr::{
    config::AppConfig as UncovRConfig, config::Environment, prelude::ApiRouter, server::Server,
};

use crate::config::{load_config, to_uncovr_config};
use crate::static_files;
use crate::templates;

/// Builder for creating web applications with sensible defaults
pub struct App {
    config: Option<UncovRConfig>,
    web_routes: Option<ApiRouter>,
    api_routes: Option<ApiRouteConfig>,
    static_config: Option<StaticConfig>,
    enable_live_reload: bool,
    environment: Option<Environment>,
    templates_path: Option<String>,
    docs_path: Option<String>,
    openapi_json_path: Option<String>,
}

/// API route configuration
pub struct ApiRouteConfig {
    pub path: String,
    pub routes: ApiRouter,
    pub docs_path: Option<String>,
    pub openapi_json_path: Option<String>,
}

/// Static files configuration
pub struct StaticConfig {
    pub serve_path: String,
    pub directory: String,
}

impl App {
    /// Create a new app with default configuration
    pub fn new() -> Self {
        Self {
            config: None,
            web_routes: None,
            api_routes: None,
            static_config: Some(StaticConfig {
                serve_path: "/public".to_string(),
                directory: "./public".to_string(),
            }),
            enable_live_reload: cfg!(debug_assertions),
            environment: None,
            templates_path: None,
            docs_path: None,
            openapi_json_path: None,
        }
    }

    /// Use custom configuration
    pub fn with_config(mut self, config: UncovRConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Load config from environment automatically
    pub fn auto_config(mut self) -> Self {
        let config = load_config();
        self.environment = Some(config.environment.clone());

        // Set templates path if configured
        if !config.templates.path.is_empty() {
            self.templates_path = Some(config.templates.path.clone());
        }

        // Set static files if configured
        if !config.templates.static_dir.is_empty() {
            self.static_config = Some(StaticConfig {
                serve_path: config.templates.static_path.clone(),
                directory: config.templates.static_dir.clone(),
            });
        }

        // Set docs paths if configured
        if !config.docs.docs_path.is_empty() {
            self.docs_path = Some(config.docs.docs_path.clone());
        }
        if !config.docs.openapi_json_path.is_empty() {
            self.openapi_json_path = Some(config.docs.openapi_json_path.clone());
        }

        self.config = Some(to_uncovr_config(&config));
        self
    }

    /// Set the environment (Development, Staging, Production)
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = Some(env);
        self
    }

    /// Set the templates directory path
    pub fn templates_path(mut self, path: impl Into<String>) -> Self {
        self.templates_path = Some(path.into());
        self
    }

    /// Set the Swagger docs path
    pub fn docs_path(mut self, path: impl Into<String>) -> Self {
        self.docs_path = Some(path.into());
        self
    }

    /// Set the OpenAPI JSON path
    pub fn openapi_json_path(mut self, path: impl Into<String>) -> Self {
        self.openapi_json_path = Some(path.into());
        self
    }

    /// Add web routes
    pub fn web(mut self, routes: ApiRouter) -> Self {
        self.web_routes = Some(routes);
        self
    }

    /// Add API routes with OpenAPI docs (uses configured paths or defaults)
    pub fn api(mut self, path: impl Into<String>, routes: ApiRouter) -> Self {
        let docs_path = self
            .docs_path
            .clone()
            .unwrap_or_else(|| "/swagger".to_string());
        let openapi_path = self
            .openapi_json_path
            .clone()
            .unwrap_or_else(|| "/openapi.json".to_string());

        self.api_routes = Some(ApiRouteConfig {
            path: path.into(),
            routes,
            docs_path: Some(docs_path),
            openapi_json_path: Some(openapi_path),
        });
        self
    }

    /// Add API routes with custom configuration
    pub fn api_with_config(
        mut self,
        path: impl Into<String>,
        routes: ApiRouter,
        docs_path: Option<impl Into<String>>,
        openapi_json_path: Option<impl Into<String>>,
    ) -> Self {
        self.api_routes = Some(ApiRouteConfig {
            path: path.into(),
            routes,
            docs_path: docs_path.map(|p| p.into()),
            openapi_json_path: openapi_json_path.map(|p| p.into()),
        });
        self
    }

    /// Disable API documentation
    pub fn api_no_docs(mut self, path: impl Into<String>, routes: ApiRouter) -> Self {
        self.api_routes = Some(ApiRouteConfig {
            path: path.into(),
            routes,
            docs_path: None,
            openapi_json_path: None,
        });
        self
    }

    /// Enable static file serving with custom paths
    pub fn static_files(
        mut self,
        serve_path: impl Into<String>,
        directory: impl Into<String>,
    ) -> Self {
        self.static_config = Some(StaticConfig {
            serve_path: serve_path.into(),
            directory: directory.into(),
        });
        self
    }

    /// Disable static file serving
    pub fn no_static_files(mut self) -> Self {
        self.static_config = None;
        self
    }

    /// Enable or disable live reload
    pub fn live_reload(mut self, enabled: bool) -> Self {
        self.enable_live_reload = enabled;
        self
    }

    /// Build and run the server
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize templates with custom path if provided
        if let Some(template_path) = &self.templates_path {
            templates::init_templates(template_path);
        }

        let mut config = self.config.unwrap_or_else(|| {
            let cfg = load_config();
            to_uncovr_config(&cfg)
        });

        // Override environment if specified
        if let Some(env) = self.environment {
            config = config.environment(env);
        }

        let mut server = Server::new().with_config(config);

        // Add web routes
        if let Some(routes) = self.web_routes {
            server = server.merge(routes);
        }

        // Add API routes
        if let Some(api_config) = self.api_routes {
            server = server.nest(&api_config.path, api_config.routes);
        }

        // Add static files
        if let Some(static_cfg) = self.static_config {
            let static_routes =
                static_files::serve_dir(&static_cfg.serve_path, &static_cfg.directory);
            server = server.merge(static_routes);
        }

        // Add live reload in development
        #[cfg(debug_assertions)]
        if self.enable_live_reload {
            use crate::templates::live_reload_layer;
            server = server.layer(live_reload_layer());
        }

        server.build().serve().await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick setup for full-stack apps (web + api)
pub async fn fullstack(
    web_routes: ApiRouter,
    api_routes: ApiRouter,
) -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .auto_config()
        .web(web_routes)
        .api("/api", api_routes)
        .serve()
        .await
}

/// Quick setup for web-only apps
pub async fn web(routes: ApiRouter) -> Result<(), Box<dyn std::error::Error>> {
    App::new().auto_config().web(routes).serve().await
}

/// Quick setup for API-only apps
pub async fn api(routes: ApiRouter) -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .auto_config()
        .api("/api", routes)
        .no_static_files()
        .serve()
        .await
}
