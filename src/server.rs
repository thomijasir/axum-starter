// use crate::modules::AppRoute;
use crate::{models::AppState, utils::HttpError};
use axum::{error_handling::HandleErrorLayer, extract::Request, response::IntoResponse};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub struct ApplicationServer;
impl ApplicationServer {
    pub async fn serve(app_state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
        // Define layered services
        let port: u16 = app_state.env.port;
        let timeout_secs = app_state.env.timeout;
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Create service builder
        let route_layer = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()) // tracing
            // .layer(from_fn(Self::request_response_logger)) // logger
            .layer(HandleErrorLayer::new(Self::handle_timeout_error)) // timeout
            .timeout(Duration::from_secs(timeout_secs))
            .layer(Self::cors_config())
            .layer(BufferLayer::<Request>::new(1024))
            .layer(RateLimitLayer::new(1024, Duration::from_secs(1)));
        // register routes
        let app = AppRoute::register()
            .with_state(app_state.clone())
            .layer(route_layer)
            .fallback(Self::handle_404);
        // launch server
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await?;
        // tracing::info!("SERVER_LAUNCH_SUCCESS: listening on {}", addr);
        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .map_err(|err| {
                // tracing::error!("SERVER_ERROR: {err}");
                err
            })?;
        Ok(())
    }
    fn cors_config() -> CorsLayer {
        CorsLayer::new()
            .allow_origin(
                constant::CORS_WHITELIST
                    .iter()
                    .map(|origin| origin.parse().expect("INVALID_CORS_ORIGIN"))
                    .collect::<Vec<_>>(),
            )
            .allow_methods(constant::METHOD_ALLOW)
            .allow_headers(constant::HEADER_ALLOW)
    }
    async fn handle_timeout_error(
        err: Box<dyn std::error::Error + Send + Sync>
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
        // tracing::info!("Shutdown signal received, starting graceful shutdown");
    }
    async fn handle_404() -> impl IntoResponse {
        HttpError::not_found("The requested resource was not found")
    }
}
