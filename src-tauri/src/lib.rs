// src/lib.rs
use tauri::Manager;
use tauri::AppHandle;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use std::sync::Mutex;
use std::{fs, path::PathBuf};
use tauri::{State};

pub struct AppState {
  pub db: Mutex<Connection>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResult {
  pub success: bool,
  pub message: String,
  pub is_first_login: bool,
}

#[derive(Deserialize)]
pub struct LoginCredentials {
  pub username: String,
  pub password: String,
}

pub fn create_app_dir() -> PathBuf {
  let config = tauri::Config::default();
  let app_dir = match tauri::api::path::app_data_dir(&config) {
      Some(dir) => dir,
      None => {
          eprintln!("Could not determine app data directory");
          // Provide a fallback path or panic
          panic!("Unable to get app data directory");
      }
  };
  
  println!("App data directory: {}", app_dir.display());
  
  if !app_dir.exists() {
      fs::create_dir_all(&app_dir).expect("Failed to create app directory");
  }
  
  app_dir
}

pub fn initialize_database(conn: &Connection) -> SqliteResult<()> {
  conn.execute(
      "CREATE TABLE IF NOT EXISTS users (
          id INTEGER PRIMARY KEY,
          username TEXT NOT NULL UNIQUE,
          password_hash TEXT NOT NULL,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      )",
      [],
  )?;
  Ok(())
}

pub fn check_first_login(conn: &Connection) -> SqliteResult<bool> {
  let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
  let count: i64 = stmt.query_row([], |row| row.get(0))?;
  Ok(count == 0)
}


#[tauri::command]
async fn register_user(
  conn: &Connection, 
  credentials: LoginCredentials
) -> Result<LoginResult, String> {
  // Generate salt
  let salt = SaltString::generate(&mut OsRng);
  
  // Hash password
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(credentials.password.as_bytes(), &salt)
      .map_err(|e| format!("Password hashing error: {}", e))?
      .to_string();
  
  // Insert the new user
  conn.execute(
      "INSERT INTO users (username, password_hash) VALUES (?, ?)",
      params![credentials.username, password_hash],
  ).map_err(|e| format!("Failed to create user: {}", e))?;
  
  Ok(LoginResult {
      success: true,
      message: "Account created successfully".to_string(),
      is_first_login: true,
  })
}

#[tauri::command]
async fn verify_login(
  conn: &Connection, 
  credentials: LoginCredentials
) -> Result<LoginResult, String> {
  // Query for the user
  let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE username = ?")
      .map_err(|e| format!("Database error: {}", e))?;
  
  let password_hash: Result<String, _> = stmt.query_row(
      params![credentials.username], 
      |row| row.get(0)
  );
  
  match password_hash {
      Ok(hash) => {
          // Parse the stored password hash
          let parsed_hash = PasswordHash::new(&hash)
              .map_err(|e| format!("Failed to parse hash: {}", e))?;
          
          // Verify password
          if Argon2::default().verify_password(credentials.password.as_bytes(), &parsed_hash).is_ok() {
              Ok(LoginResult {
                  success: true,
                  message: "Login successful".to_string(),
                  is_first_login: false,
              })
          } else {
              Ok(LoginResult {
                  success: false,
                  message: "Invalid password".to_string(),
                  is_first_login: false,
              })
          }
      },
      Err(_) => {
          Ok(LoginResult {
              success: false,
              message: "User not found".to_string(),
              is_first_login: false,
          })
      }
  }
}




