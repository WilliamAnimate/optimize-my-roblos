// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path, fs};

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn optimize() {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => {
            // TODO: error handling by calling the js?
            eprintln!("Failed to get APPDATA path: {}", err);
            return;
        }
    };

    let folder_path = format!("{}\\Roblox\\Versions", local_appdata_path);

    let full_path = std::env::current_dir().unwrap().join(folder_path);
    let result = find_roblox_exe(&full_path);

    match result {
        Some(result_folder_name) => {
            let rlbx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path); // oh god where tf are we im losing it to rust
            // i suppose i owe you an explaination:
            // so basically, folder_path doesnt want to be .clone()'d, nor is it copyable or something, so im gonna repeat this code here
            // if you modify folder_path then modify this too lmao
            // i **SERIOUSLY** need to read the rustbook again

            println!("Found RobloxPlayerBeta.exe in folder: {}", result_folder_name);

            println!("{}", rlbx_path);

            if let Err(e) = fs::create_dir_all(&rlbx_path) {
                eprintln!("Error creating folder: {}", e);
                return;
            }

            let client_settings = include_bytes!("ClientAppSettings.json");

            if let Err(e) = fs::write(format!("{}\\ClientAppSettings.json", rlbx_path), client_settings) {
                eprintln!("Error creating file: {}", e);
                return;
            }
        }
        None => eprintln!("RobloxPlayerBeta not found... do you have the game installed?"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_version,
            optimize
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO: refactor and fix indentation hell
fn find_roblox_exe(directory: &Path) -> Option<String> {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(folder_name) = find_roblox_exe(&path) {
                        return Some(folder_name);
                    }
                } else if path.is_file() && path.file_name().unwrap_or_default() == "RobloxPlayerBeta.exe" {
                    if let Some(parent) = path.parent() {
                        if let Some(folder_name) = parent.file_name() {
                            return Some(folder_name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    None
}