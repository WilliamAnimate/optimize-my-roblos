// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod console;

use crate::console::cli_attach_to_console;
use std::{path::Path, fs, env};

/// returns the app version, if this is not self-documenting code then I don't know what is.
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// the main function for optimizing
///
/// this function requires the function `find_roblox_exe`, it'll do the rest, assuming `ClientAppSettings.json` is included in the same directory as this file.
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

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path);

            if let Err(err) = fs::create_dir_all(&rblx_path) {
                eprintln!("Error creating folder: {}", err);
                return;
            }

            let client_settings = include_bytes!("ClientAppSettings.json");

            if let Err(err) = fs::write(format!("{}\\ClientAppSettings.json", rblx_path), client_settings) {
                eprintln!("Error creating file: {}", err);
                return;
            }
        }
        None => eprintln!("RobloxPlayerBeta not found... do you have the game installed?"),
    }
}

/// this is where the program's entrypoint is. if this is not self-documenting code, then I don't know what is.
fn main() {
    let args: Vec<String> = env::args().collect();

    // assume CLI first
    // look i know tauri has their own feature for CLIs but i like seeing the world burn
    if args.len() > 1 {
        cli_attach_to_console();

        println!("Optimize_my_Roblos version {}", get_version());
        println!("Keep in mind that the updater is NOT present in CLI mode.");

        // for (index, _argument) in args.iter().enumerate().skip(1) {
        match args.get(1).map(|arg| arg.as_str()) {
            Some("-optimizenow") | Some("-o") => {
                optimize();
                println!("Optimization done!");
            },
            Some("-help") | Some("-h") => {
                println!("=== HELP ===");
                println!("If this is an actual plea for help, this isn't the right place.");
                println!("otherwise, if you need help with CLI mode, then this is the right place");
                println!("-optimizenow -o          runs the optimizer from CLI mode");
                println!("-help -h                 shows the help menu, looks like you already figured this out");
            }
            _ => {
                println!("invalid parameter, use -help for help. please note that slash wont help you because im a terrible person");
            }
        }
        // }

        println!("press any key to exit");
        std::process::exit(420);
    }

    // no CLI, start the GUI.
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_version,
            optimize
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// finds the roblox working directory. this code assumes that:
///
/// - you have supplied the directory to search, keep in mind that **there is _no_ fallback directory if it isn't supplied**
/// - roblox does not modify the way "instances" are installed. this code works as of 11/11/2023
/// - you have a match statement to see the result
///
/// returns: the full path of the working directory, if its not found, it will return `None`.
///
/// for me (11/11/2023), it returns `C:\Users\willi\AppData\Local\Roblox\Versions\version-3aba366803e44f0e`
// TODO: fix indentation hell
fn find_roblox_exe(directory: &Path) -> Option<String> {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(folder_name) = find_roblox_exe(&path) {
                    return Some(folder_name);
                }
            } else if path.is_file() && path.file_name() == Some("RobloxPlayerBeta.exe".as_ref()) {
                if let Some(parent) = path.parent() {
                    if let Some(folder_name) = parent.file_name() {
                        return Some(folder_name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
}
