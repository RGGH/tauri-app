// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EdgeKraft", name)
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
