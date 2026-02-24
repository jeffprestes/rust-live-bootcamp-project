use axum::{body::Body, extract::Request, response::Response};
use tracing::{Level, Span};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use color_eyre::eyre::Result;

pub fn init_tracing() -> Result<()>{
  let fmt_layer = fmt::layer().compact();
  let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

  tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();

  Ok(())
}
pub fn make_span_with_request_id(request: &Request<Body>) -> Span {
  let request_id = uuid::Uuid::new_v4();
  tracing::span!(
    Level::INFO,
    "[WEBREQUEST]",
    method = display(request.method()),
    uri = display(request.uri()),
    request_id = display(request_id)
  )
}

pub fn on_request(_request: &Request<Body>, _span: &Span) {
  tracing::event!(Level::INFO, "[REQUEST START]");
}

pub fn on_response(response: &Response, latency: std::time::Duration, _span: &Span) {
  let status = response.status();
  let status_code = status.as_u16();
  let status_code_class = status_code / 100;
  match status_code_class {
    4..5 => tracing::event!(Level::ERROR, latency = ?latency, status = status_code, "[REQUEST END]"),
    _ => tracing::event!(Level::INFO, latency = ?latency, status = status_code, "[REQUEST END]"),
  }
}