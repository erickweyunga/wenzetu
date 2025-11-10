//! Prelude module - commonly used imports
//!
//! Import everything you need with:
//! ```rust
//! use wenzetu::prelude::*;
//! ```

// Re-export from uncovr
pub use uncovr::prelude::*;

// Re-export from wenzetu
pub use crate::app::{self, App};
pub use crate::config::{AppConfig, load_config, to_uncovr_config};
pub use crate::helpers;
pub use crate::templates::{TEMPLATES, render};
pub use crate::{context, static_files};

// Re-export Tera for context building
pub use tera::Context;

#[cfg(feature = "live-reload")]
pub use crate::templates::live_reload_layer;
