use crate::{
    mailers::auth::AuthMailer,
    models::{_entities::users, otp_codes::Model as OtpModel, users::Model as UserModel},
    views::auth::CurrentResponse,
};
use axum::http::header::{HeaderMap, SET_COOKIE};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestOtpParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyOtpParams {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
}

#[debug_handler]
async fn request_otp(
    State(ctx): State<AppContext>,
    Json(params): Json<RequestOtpParams>,
) -> Result<Response> {
    let user = UserModel::find_or_create_by_email(&ctx.db, &params.email).await?;
    let code = OtpModel::create_otp(&ctx.db, user.id).await?;
    tracing::info!("Generated OTP for {}: {}", user.email, code);
    if let Err(err) = AuthMailer::send_otp(&ctx, &user, &code).await {
        tracing::warn!("Failed to send OTP email: {}, OTP code: {}", err, code);
    }
    format::json(AuthResponse {
        message: "OTP sent".into(),
    })
}

#[debug_handler]
async fn verify_otp(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyOtpParams>,
) -> Result<Response> {
    let user = UserModel::find_or_create_by_email(&ctx.db, &params.email).await?;

    if !OtpModel::verify_otp(&ctx.db, user.id, &params.code).await? {
        return unauthorized("Invalid or expired OTP");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("Token generation failed"))?;

    let cookie_str = format!(
        "token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
        token, jwt_secret.expiration
    );

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie_str.parse().unwrap());

    Ok((headers, format::json(CurrentResponse::new(&user))).into_response())
}

#[debug_handler]
async fn logout() -> Result<Response> {
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        "token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax"
            .parse()
            .unwrap(),
    );
    Ok((
        headers,
        format::json(AuthResponse {
            message: "Logged out".into(),
        }),
    )
        .into_response())
}

#[debug_handler]
async fn me(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/request-otp", post(request_otp))
        .add("/verify-otp", post(verify_otp))
        .add("/logout", post(logout))
        .add("/me", get(me))
}
