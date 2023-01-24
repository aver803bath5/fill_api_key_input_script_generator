#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fs::File;
use serde::Serialize;
use sqlx::mssql::{MssqlPoolOptions, MssqlRow};
use sqlx::{Row};

#[derive(Serialize)]
struct ShopApiProfile {
    api_key: String,
    salt: String,
    token: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(market: &str, shop_id: &str) -> Result<ShopApiProfile, ()> {
    // read connection string from config file
    let file = File::open("connection_strings.json").unwrap();
    let config: serde_json::Value = serde_json::from_reader(file).unwrap();

    // read different connection string for different market
    let conn_str = config[market.to_uppercase()].as_str().unwrap();
    let pool = MssqlPoolOptions::new()
        .max_connections(1)
        .idle_timeout(std::time::Duration::from_secs(3))
        .connect(&conn_str).await.unwrap();

    Ok(sqlx::query(r#"
    USE ERPDB;
        SELECT SupplierApiProfile_Key, SupplierApiProfile_SaltKey, SupplierApiProfile_Token
    FROM dbo.SupplierApiProfile (NOLOCK)
             INNER JOIN dbo.Shop ON Shop_ValidFlag = 1 AND Shop_SupplierId = SupplierApiProfile_SupplierId
    WHERE SupplierApiProfile_ValidFlag = 1
      AND Shop_Id = @P1;
        "#)
        .bind(shop_id)
        .map(|row: MssqlRow| ShopApiProfile {
            api_key: row.try_get("SupplierApiProfile_Key").unwrap_or(String::new()),
            salt: row.try_get("SupplierApiProfile_SaltKey").unwrap_or(String::new()),
            token: row.try_get("SupplierApiProfile_Token").unwrap_or(String::new()),
        })
        .fetch_one(&pool)
        .await
        .unwrap_or(ShopApiProfile {
            api_key: String::new(),
            salt: String::new(),
            token: String::new(),
        }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
