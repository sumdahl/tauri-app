use tauri::State;

use crate::{
    error::AppError,
    services::auth::{self, SigninRequest, SigninResponse, SignupRequest, SignupResponse},
    state::AppState,
};

#[tauri::command]
pub async fn signup(
    fullname: String,
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<SignupResponse, AppError> {
    let request = SignupRequest {
        fullname,
        email,
        password,
    };

    match &state.db {
        Some(db) => auth::signup(request, db).await,
        None => auth::signup_http(&state.http, &state.backend_url, request).await,
    }
}

#[tauri::command]
pub async fn signin(
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<SigninResponse, AppError> {
    let request = SigninRequest { email, password };

    match &state.db {
        Some(db) => auth::signin(request, db).await,
        None => auth::signin_http(&state.http, &state.backend_url, request).await,
    }
}
