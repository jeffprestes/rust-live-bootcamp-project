use axum::{Router, routing::{get, post}, response::Html};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};
use std::sync::Arc;

pub(crate) mod signup;
pub(crate) mod login;
pub(crate) mod logout;
pub mod verify_token;
pub mod verify_2fa;
pub mod cleanup;
/* 
mod verify_2fa;
mod verify_token;
*/

// re-export items from sub-modules
pub use signup::signup;
pub use login::login;
pub use logout::logout;
pub use verify_token::*;
pub use verify_2fa::*;
pub use cleanup::cleanup;

use crate::app_state::AppState;

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
        .route("/cleanup", get(cleanup))
        .route("/cleanup/{email}", get(cleanup::cleanup_single_user))
        .route("/verify-token", post(verify_token))
        .route("/verify-2fa", post(verify_2fa))
        .fallback_service(assets_dir)
        .layer(cors)
        .with_state(app_state);
    router
}

async fn heartbeat_handler() -> Html<&'static str> {
    // Done: Update this to a custom message! [done]
    Html("online")
}