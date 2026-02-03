use axum::{Router, routing::{get, post}, response::Html};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};
use std::sync::Arc;

pub(crate) mod signup;
pub(crate) mod login;
pub(crate) mod logout;
/* 
mod verify_2fa;
mod verify_token;
*/

// re-export items from sub-modules
pub use signup::signup;
pub use login::login;
pub use logout::logout;

use crate::app_state::AppState;
/*
pub use verify_2fa::*;
pub use verify_token::*;
 */

pub fn generate_routes(app_state: AppState, cors: CorsLayer) -> Router {
    // This function can be used to initialize or configure routes if needed in the future.
    let app_state = Arc::new(app_state);
    let assets_dir = ServeDir::new("assets")
    .not_found_service(ServeFile::new("assets/index.html"));
    let router = Router::new()
        .route("/heartbeat", get(heartbeat_handler))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/logout", get(logout))
        .fallback_service(assets_dir)
        .layer(cors)
        .with_state(app_state);
        //TODO: Add routes for verify-2fa, and verify-token
    router
}

async fn heartbeat_handler() -> Html<&'static str> {
    // Done: Update this to a custom message! [done]
    Html("online")
}