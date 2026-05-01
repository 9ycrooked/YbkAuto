use crate::login::{self, CompletionResult, DashboardState, SessionState};
use tauri::AppHandle;

#[tauri::command]
pub async fn bootstrap_session(app: AppHandle) -> Result<SessionState, String> {
    login::bootstrap_session(&app).await
}

#[tauri::command]
pub async fn login_command(
    app: AppHandle,
    username: String,
    ciphertext: String,
) -> Result<SessionState, String> {
    login::login_and_build_session(&app, username, ciphertext).await
}

#[tauri::command]
pub async fn refresh_dashboard(app: AppHandle) -> Result<DashboardState, String> {
    login::refresh_dashboard(&app).await
}

#[tauri::command]
pub fn logout_command(app: AppHandle) -> Result<(), String> {
    login::logout(&app)
}

#[tauri::command]
pub async fn complete_course_resources(
    app: AppHandle,
    ccid: String,
) -> Result<CompletionResult, String> {
    login::complete_course_resources(&app, &ccid).await
}
