#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

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
    let db_account = "sa";
    let db_password = "yourStrong@Passw0rd";
    let conn_str = format!("mssql://{}:{}@localhost:1433", db_account, db_password);
    let pool = MssqlPoolOptions::new()
        .max_connections(1)
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
