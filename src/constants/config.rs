use axum::http::{HeaderName, Method, header};
// Configuration Path
pub const CONFIG_CONSTANT: &str = "./config/constant.toml";

// Global Constants
pub const CACHE_TIMEOUT: u64 = 3600; // 1 hour default cache
pub const METHOD_ALLOW: [Method; 4] = [Method::GET, Method::POST, Method::PUT, Method::DELETE];
pub const HEADER_ALLOW: [HeaderName; 2] = [header::CONTENT_TYPE, header::ACCEPT];
pub const CORS_WHITELIST: [&str; 2] = ["http://localhost:5000", "http://localhost:8080"];
pub const IMAGE_TYPES_SUPPORT: [&str; 3] = ["jpg", "jpeg", "png"];
pub const VIDEO_TYPES_SUPPORT: [&str; 1] = ["mp4"];
pub const DOCUMENT_TYPES_SUPPORT: [&str; 8] =
    ["pdf", "docx", "json", "txt", "doc", "html", "htm", "md"];

// Outbox Event Constants
pub const OUTBOX_EVENT_PROPERTY_LIKE: &str = "property.like";
pub const OUTBOX_EVENT_PROPERTY_SUPERLIKE: &str = "property.superlike";
pub const OUTBOX_EVENT_PROPERTY_DISLIKE: &str = "property.dislike";
pub const OUTBOX_EVENT_AGGREGATE_PROPERTY: &str = "property";
