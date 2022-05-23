#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn log(context: String, message: String) {
    println!("{} {}", context, message);
}

#[tauri::command]
fn test() {
    println!("THIS IS A TEST");
}
