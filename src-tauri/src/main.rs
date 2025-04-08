// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use client::llm_get;

mod client;

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EdgeKraft", name)
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn my_custom_command2(invoke_message: String) {
    // Process the message
    let res = format!("✅ This will pass to the LLM: {}", invoke_message);
    println!("{:?}", res);
}

fn main() {
    // let _ = llm_get(); <--- todo!
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);


    tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
        match event.menu_item_id() {
          "quit" => {
            std::process::exit(0);
          }
          "close" => {
            event.window().close().unwrap();
          }
          _ => {}
        }
      })
        .invoke_handler(tauri::generate_handler![greet,my_custom_command,my_custom_command2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
