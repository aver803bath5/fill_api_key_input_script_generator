#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;

#[derive(Serialize)]
struct ShopApiProfile<'a> {
    api_key: &'a str,
    salt: &'a str,
    token: &'a str,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn search<'a>(market: &'a str, shop_id: &'a str) -> ShopApiProfile<'a> {
    ShopApiProfile { api_key: "abd", salt: "ddd", token: "token" }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
