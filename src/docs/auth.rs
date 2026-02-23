use crate::modules::auth::model::{AuthTokens, LoginRequest, RefreshRequest, RegisterRequest};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthTokens),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists")
    )
)]
#[allow(dead_code)]
pub fn auth_register() {}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthTokens),
        (status = 401, description = "Invalid credentials")
    )
)]
#[allow(dead_code)]
pub fn auth_login() {}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AuthTokens),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
#[allow(dead_code)]
pub fn auth_refresh() {}
