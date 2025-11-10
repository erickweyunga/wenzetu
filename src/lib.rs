//! Wenzetu - Helper crate for uncovr projects
//!

pub mod app;
pub mod config;
pub mod helpers;
pub mod prelude;
pub mod static_files;
pub mod templates;

// Re-export commonly used types
pub use uncovr;

pub use app::App;
pub use config::AppConfig;
pub use templates::render;
