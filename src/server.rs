use crate::{
  constants::{HEADER_ALLOW, METHOD_ALLOW},
  models::AppState,
  modules::AppRoutes,
  utils::HttpError,
};
use axum::{
  error_handling::HandleErrorLayer,
  extract::Request,
  http::HeaderValue,
  response::IntoResponse,
  routing::any,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{
  cors::CorsLayer,
  request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
  services::ServeDir,
  trace::TraceLayer,
};

pub struct AppServer;
impl AppServer {
  pub async fn serve(app_state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = app_state.env.port;
    let timeout_secs = app_state.env.timeout;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let cors = Self::cors_config(&app_state.env.cors_origins);

    let route_layer = ServiceBuilder::new()
      // Assign a unique request ID to every request
      .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
      .layer(TraceLayer::new_for_http())
      .layer(HandleErrorLayer::new(Self::handle_timeout_error))
      .timeout(Duration::from_secs(timeout_secs))
      .layer(cors)
      .layer(BufferLayer::<Request>::new(1024))
      .layer(RateLimitLayer::new(1024, Duration::from_secs(1)))
      // Propagate request ID back in the response
      .layer(PropagateRequestIdLayer::x_request_id());

    let serve_dir = ServeDir::new("public").fallback(any(Self::handle_404));

    let app = AppRoutes::build(app_state.clone())
      .fallback_service(serve_dir)
      .layer(route_layer);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
      .with_graceful_shutdown(Self::shutdown_signal())
      .await?;
    Ok(())
  }

  fn cors_config(origins: &[String]) -> CorsLayer {
    let allowed: Vec<HeaderValue> = origins
      .iter()
      .filter_map(|o| o.parse::<HeaderValue>().ok())
      .collect();
    CorsLayer::new()
      .allow_origin(allowed)
      .allow_methods(METHOD_ALLOW)
      .allow_headers(HEADER_ALLOW)
  }

  async fn handle_timeout_error(
    err: Box<dyn std::error::Error + Send + Sync>,
  ) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
      HttpError::timeout("REQUEST_TIMED_OUT")
    } else {
      HttpError::server_error("UNEXPECTED_ERROR_OCCURRED")
    }
  }

  async fn shutdown_signal() {
    let ctrl_c = async {
      tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
      tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
  }

  async fn handle_404() -> impl IntoResponse {
    HttpError::not_found("The requested resource was not found")
  }
}
