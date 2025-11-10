//! Static file serving utilities

use tower_http::services::ServeDir;
use uncovr::prelude::ApiRouter;
use uncovr::routing::get_service;

/// Add static file routes to the server.
///
/// # Example
/// ```rust
/// use wenzetu::static_files::add_static_routes;
///
/// let router = add_static_routes();
/// ```
pub fn add_static_routes() -> ApiRouter {
    let public = ServeDir::new("./public");

    ApiRouter::new().nest_service("/public", get_service(public))
}

/// Add custom static file route.
///
/// # Example
/// ```rust
/// use wenzetu::static_files::serve_dir;
///
/// let router = serve_dir("/assets", "./public/assets");
/// ```
pub fn serve_dir(path: &str, directory: &str) -> ApiRouter {
    let serve = ServeDir::new(directory);
    ApiRouter::new().nest_service(path, get_service(serve))
}
