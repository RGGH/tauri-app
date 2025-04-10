// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri_app::{create_app_dir, initialize_database, AppState};
use client::llm_get;
mod client;
use tauri::Manager;

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EdgeKraft™", name)
}

#[tauri::command]
fn my_custom_command(app_handle: tauri::AppHandle) {
    tauri::api::shell::open(&app_handle.shell_scope(), "https://example.com".to_string(), None).unwrap();
}


#[tauri::command]
async fn ask_llm(invoke_message: String) -> String {
    let res = llm_get(invoke_message).await;
    println!("{}", res);
    res
}

fn main() {
    let app_dir = create_app_dir();
    let db_path = app_dir.join("app.db");
    let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");
    initialize_database(&conn).expect("Failed to initialize database");
    
    let quit = CustomMenuItem::new("login".to_string(), "Login");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("Start", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        //.add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "login" => {
                    // Create a new window in the existing application instead of a new Builder
                    let app_handle = event.window().app_handle();
                    
                    // Create a new window within the existing application
                    tauri::WindowBuilder::new(
                        &app_handle,
                        "local", // window identifier
                        tauri::WindowUrl::App("edgekraft.html".into())
                    )
                    .build()
                    .expect("Failed to create login window");
                }
                "close" => {
                    event.window().close().unwrap();
                }
                _ => {
                    println!("You need to log in")
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, my_custom_command, ask_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}