use axum::{Router, routing::{get, post}, response::Html};
use tower_http::services::{ServeDir, ServeFile};

pub(crate) mod signup;
pub(crate) mod login;
/* 
mod login;
mod logout;
mod verify_2fa;
mod verify_token;
*/

// re-export items from sub-modules
pub use signup::signup;
pub use login::login;

use crate::app_state::AppState;
/*
pub use logout::*;
pub use verify_2fa::*;
pub use verify_token::*;
 */

pub fn generate_routes(app_state: AppState) -> Router {
    // This function can be used to initialize or configure routes if needed in the future.
    let assets_dir = ServeDir::new("assets")
    .not_found_service(ServeFile::new("assets/index.html"));
    let router = Router::new()
        .route("/heartbeat", get(heartbeat_handler))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .fallback_service(assets_dir)
        .with_state(app_state.into());
        //TODO: Add routes for logout, verify-2fa, and verify-token
    router
}

async fn heartbeat_handler() -> Html<&'static str> {
    // Done: Update this to a custom message! [done]
    Html("online")
}