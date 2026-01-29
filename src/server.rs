use crate::modules::AppRoute;
use crate::{AppState, constant, utils::errors::HttpError};
use axum::body::{Body, to_bytes};
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::middleware::from_fn;
use axum::response::Response;
use axum::{error_handling::HandleErrorLayer, extract::Request, response::IntoResponse};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use ulid::Ulid;

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
        tracing::info!("SERVER_LAUNCH_SUCCESS: listening on {}", addr);
        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .map_err(|err| {
                tracing::error!("SERVER_ERROR: {err}");
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
    // Middleware that records requests resulting in client or server error responses
    // async fn request_response_logger(
    //     req: Request,
    //     next: Next,
    // ) -> Response {
    //     // Generate per-request trace id
    //     let trace_id = Ulid::new();

    //     let method = req.method().clone();
    //     let uri = req.uri().clone();
    //     let mut response = next.run(req).await;
    //     // attach trace id header
    //     match HeaderValue::from_str(&trace_id.to_string()) {
    //         Ok(header_value) => {
    //             response.headers_mut().insert("x-trace-id", header_value);
    //         }
    //         Err(err) => {
    //             // Proceed without setting the header but log the issue for observability
    //             tracing::error!(
    //                 trace_id = %trace_id,
    //                 error = %err,
    //                 "INVALID_TRACE_ID_HEADER_VALUE"
    //             );
    //         }
    //     }

    //     let status = response.status();

    //     if status.is_client_error() || status.is_server_error() {
    //         // take the body to bytes
    //         let (parts, body) = response.into_parts();
    //         // 16 * 1024 means 16 KiB cap for logged bodies
    //         match to_bytes(body, 16 * 1024).await {
    //             Ok(bytes) => {
    //                 let body_str = String::from_utf8_lossy(&bytes);
    //                 tracing::error!(
    //                     %method,
    //                     trace_id = %trace_id,
    //                     path = %uri.path(),
    //                     status = %status.as_u16(),
    //                     body = %body_str,
    //                     "HTTP_ERROR_RESPONSE"
    //                 );
    //                 // build response back with same body
    //                 response = Response::from_parts(parts, Body::from(bytes));
    //             }
    //             Err(err) => {
    //                 tracing::error!(
    //                     %method,
    //                     trace_id = %trace_id,
    //                     path = %uri.path(),
    //                     status = %status.as_u16(),
    //                     error = %err,
    //                     "FAILED_TO_READ_RESPONSE_BODY"
    //                 );
    //                 // body already consumed; return empty
    //                 response = Response::from_parts(parts, Body::empty());
    //             }
    //         }
    //     } else {
    //         tracing::info!(
    //             %method,
    //             trace_id = %trace_id,
    //             path = %uri.path(),
    //             status = %status.as_u16(),
    //             "HTTP_SUCCESS_RESPONSE"
    //         );
    //     }
    //     response
    // }

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
        tracing::info!("Shutdown signal received, starting graceful shutdown");
    }
    async fn handle_404() -> impl IntoResponse {
        HttpError::not_found("The requested resource was not found")
    }
}
