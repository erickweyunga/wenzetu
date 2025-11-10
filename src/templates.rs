//! Template rendering with Tera
//!
//! Provides a global Tera instance with hot-reload in development and
//! comprehensive error handling with detailed debug pages.

use std::sync::{Arc, LazyLock, RwLock};
use tera::{Context, Tera};

#[cfg(feature = "live-reload")]
use std::time::Duration;
#[cfg(feature = "live-reload")]
use tera_hot_reload::watch;
#[cfg(feature = "live-reload")]
use tower_livereload::Reloader;

/// Live reload reloader (active in development only).
#[cfg(feature = "live-reload")]
pub static LIVE_RELOADER: LazyLock<Reloader> = LazyLock::new(Reloader::new);

/// Tracks the latest Tera template initialization or reload error.
pub static TERA_INIT_ERROR: LazyLock<RwLock<Option<String>>> = LazyLock::new(|| RwLock::new(None));

/// Template path configuration
pub static TEMPLATE_PATH: RwLock<String> = RwLock::new(String::new());

/// Initialize templates with a custom path
pub fn init_templates(path: impl Into<String>) {
    let path_str = path.into();
    if let Ok(mut template_path) = TEMPLATE_PATH.write() {
        *template_path = path_str;
    }
}

/// Get the configured template path
fn get_template_path() -> String {
    if let Ok(path) = TEMPLATE_PATH.read() {
        if !path.is_empty() {
            return path.clone();
        }
    }
    "templates/**/*".to_string()
}

/// Global Tera instance shared across the application.
pub static TEMPLATES: LazyLock<Arc<RwLock<Tera>>> = LazyLock::new(|| {
    let template_path = get_template_path();
    let mut tera = match Tera::new(&template_path) {
        Ok(t) => t,
        Err(err) => {
            if let Ok(mut lock) = TERA_INIT_ERROR.write() {
                *lock = Some(err.to_string());
            }
            Tera::default()
        }
    };

    // Configure auto-escaping for security
    tera.autoescape_on(vec![".html", ".htm", ".xml", ".svg"]);

    let tera_ref = Arc::new(RwLock::new(tera));

    // Enable live reload during development
    #[cfg(feature = "live-reload")]
    {
        let watch_ref = Arc::clone(&tera_ref);
        let watch_path = get_template_path().replace("/**/*", "");
        let _debouncer = watch(
            move || {
                if let Ok(mut tera_guard) = watch_ref.write() {
                    if let Err(err) = tera_guard.full_reload() {
                        if let Ok(mut lock) = TERA_INIT_ERROR.write() {
                            *lock = Some(err.to_string());
                        }
                    } else {
                        if let Ok(mut lock) = TERA_INIT_ERROR.write() {
                            *lock = None;
                        }
                    }
                }
                LIVE_RELOADER.reload();
            },
            Duration::from_millis(100),
            vec![watch_path],
        );
        std::mem::forget(_debouncer);
    }

    tera_ref
});

/// Render a template with the given context.
///
/// # Example
/// ```rust
/// use wenzetu::{render, context};
///
/// let html = render("index.html", &context! {
///     title: "Home",
///     user: "John",
/// });
/// ```
pub fn render(name: &str, context: &Context) -> String {
    match TEMPLATES.read() {
        Ok(tera_guard) => {
            // Check for initialization errors
            if let Ok(err_lock) = TERA_INIT_ERROR.read() {
                if let Some(init_err) = err_lock.as_ref() {
                    eprintln!("Template initialization error: {}", init_err);
                    return format!(
                        "<!DOCTYPE html><html><body><h1>Template Error</h1><pre>{}</pre></body></html>",
                        html_escape::encode_text(init_err)
                    );
                }
            }

            match tera_guard.render(name, context) {
                Ok(html) => html,
                Err(err) => {
                    eprintln!("Template render error in '{}': {}", name, err);
                    format!(
                        "<!DOCTYPE html><html><body><h1>Template Render Error</h1>\
                        <p>Template: {}</p><pre>{}</pre></body></html>",
                        html_escape::encode_text(name),
                        html_escape::encode_text(&err.to_string())
                    )
                }
            }
        }
        Err(err) => {
            eprintln!("Template lock error: {}", err);
            format!(
                "<!DOCTYPE html><html><body><h1>Template Lock Error</h1><pre>{}</pre></body></html>",
                html_escape::encode_text(&err.to_string())
            )
        }
    }
}

/// Create a Tera context from key-value pairs.
///
/// # Example
/// ```rust
/// use wenzetu::context;
///
/// let ctx = context! {
///     name: "World",
///     count: 42,
/// };
/// ```
#[macro_export]
macro_rules! context {
    ($($key:ident: $value:expr),* $(,)?) => {{
        let mut ctx = tera::Context::new();
        $(
            ctx.insert(stringify!($key), &$value);
        )*
        ctx
    }};
}

/// Re-export the live reload layer for easy integration
#[cfg(feature = "live-reload")]
pub fn live_reload_layer() -> tower_livereload::LiveReloadLayer {
    use tower_livereload::LiveReloadLayer;
    LiveReloadLayer::new()
}
