mod api;
mod commands;
mod login;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap_session,
            commands::login_command,
            commands::refresh_dashboard,
            commands::logout_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
