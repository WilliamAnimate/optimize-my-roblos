// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod console;

use crate::console::cli_attach_to_console;
use std::{path::Path, fs, env};

use winreg::{enums::*, RegKey};

/// returns the app version, if this is not self-documenting code then I don't know what is.
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_localappdata_path() -> String {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => {
                        // ` as first char to indicate error
            return format!("`{}", err)
        }
    };

    local_appdata_path
}

fn apply_clientappsettings_json(client_settings: &[u8]) -> String {
    let local_appdata_path = get_localappdata_path();

    let first_char = local_appdata_path.chars().next();
    if first_char == Some('`') {
        return format!("Unable to get the value of the localappdata environment variable: {}", local_appdata_path)
    }

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path);

            if let Err(err) = fs::create_dir_all(&rblx_path) {
                return format!("Error creating folder: {}", err);
            }

            if let Err(err) = fs::write(format!("{}\\ClientAppSettings.json", rblx_path), client_settings) {
                return format!("Error creating file: {}", err).to_string();
            }
        }
        None => {
            return String::from("RobloxPlayerBeta not found... do you have the game installed?");
        }
    }

    String::from("we gud")
}

/// the main function for optimizing
///
/// this function requires the function `find_roblox_exe`, it'll do the rest, assuming `ClientAppSettings.json` is included in the same directory as this file.
#[tauri::command]
fn optimize() -> String {
    apply_clientappsettings_json(include_bytes!("ClientAppSettings.json"))
}

#[tauri::command]
fn optimize_alt_tweaks() -> String {
    apply_clientappsettings_json(include_bytes!("ClientAppSettingsAlt.json"))
}

#[tauri::command]
fn optimize_minimal_novulkan() -> String {
    apply_clientappsettings_json(include_bytes!("ClientAppMinimalNoVulkan.json"))
}

#[tauri::command]
fn optimize_gpu_settings() -> String {
    println!("This code has no error handling! Press any key to execute.");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let local_appdata_path = get_localappdata_path();

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\RobloxPlayerBeta.exe", local_appdata_path);

            let path_dx11 = Path::new("Software\\Microsoft\\DirectX\\UserGpuPreferences");
            let path_app_compat_flags = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers");
            // let perf_options = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RobloxPlayerBeta.exe\\PerfOptions");

            let Ok((key_dx11, _)) = hkcu.create_subkey(&path_dx11) else {panic!("gg")};
            let Ok((key_app_compat_flags, _)) = hkcu.create_subkey(&path_app_compat_flags) else {panic!("gg")};
            // let (key_perf_options, _) = hklm.create_subkey(&perf_options);

            // note: clone the rblx_path so it doesn't mess things up
            let _ = key_dx11.set_value(rblx_path.clone(), &"GpuPreference=2;");
            let _ = key_app_compat_flags.set_value(rblx_path, &"~ DISABLEDXMAXIMIZEDWINDOWEDMODE");
            // key_perf_options.set_value("CpuPriorityClass", &"3");

            String::from("we gud")
        }
        None => String::from("RobloxPlayerBeta not found... do you have the game installed?")
    }

    // Ok(())
}

#[tauri::command]
fn unoptimize() -> String {
    let local_appdata_path = get_localappdata_path();

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            match fs::remove_dir_all(format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path)) {
                Ok(_) => String::from("we gud"),
                Err(err) => return format!("wtf? {}", err)
            }
        }
        None => {
            String::from("RobloxPlayerBeta not found... do you have the game installed?")
        }
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
            optimize,
            optimize_alt_tweaks,
            optimize_minimal_novulkan,
            optimize_gpu_settings,
            unoptimize
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
