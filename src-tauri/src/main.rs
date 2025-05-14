// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bip39::{Mnemonic, Language};

#[tauri::command]
fn generate_mnemonic() -> Result<String, String> {
    let entropy = [0u8; 16];
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).map_err(|e| e.to_string())?;
    Ok(mnemonic.to_string())
}

#[tauri::command]
fn verify_mnemonic(phrase: String) -> bool {
    Mnemonic::parse_in(Language::English, &phrase).is_ok()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_mnemonic, verify_mnemonic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}