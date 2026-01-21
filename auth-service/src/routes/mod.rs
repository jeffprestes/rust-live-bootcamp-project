use axum::{Router, routing::{get, post}, response::Html};
use tower_http::services::{ServeDir, ServeFile};

pub(crate) mod signup;
/* 
mod login;
mod logout;
mod verify_2fa;
mod verify_token;
*/

// re-export items from sub-modules
pub use signup::signup;
/*
pub use login::*;
pub use logout::*;
pub use verify_2fa::*;
pub use verify_token::*;
 */

pub fn generate_routes() -> Router {
    // This function can be used to initialize or configure routes if needed in the future.
    let assets_dir = ServeDir::new("assets")
    .not_found_service(ServeFile::new("assets/index.html"));
    let router = Router::new()
        .fallback_service(assets_dir)
        .route("/heartbeat", get(heartbeat_handler))
        .route("/signup", post(signup));
        //TODO: Add routes for login, logout, verify-2fa, and verify-token
    router
}

async fn heartbeat_handler() -> Html<&'static str> {
    // Done: Update this to a custom message! [done]
    Html("online")
}