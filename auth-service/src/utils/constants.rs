pub const JWT_COOKIE_NAME: &str = "jwt-rust-live-bootcamp";
pub const JWT_SECRET: &str = "minha_chave_secreta_super_secreta"; 
pub mod prod {
  pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}
pub mod dev {
  pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
