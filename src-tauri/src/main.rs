// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod console;

// use crate::console::cli_attach_to_console;
use std::{path::Path, fs, env, sync::Mutex};

use winreg::{enums::*, RegKey};

use lazy_static::lazy_static;

lazy_static!(
    static ref LAST_ERROR: Mutex<String> = Mutex::new(String::from("No error details were set."));
    static ref LOCALAPPDATA_PATH: Mutex<String> = Mutex::new(init_get_local_appdata_path());
);

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
fn find_roblox_exe(directory: &std::path::Path) -> Option<String> {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if fs::read_dir(&path).map(|mut dir| dir.any(|entry| entry.as_ref().map_or(false, |file| file.file_name() == "RobloxPlayerBeta.exe"))).unwrap()
                {
                    return Some(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }
    }
    None
}

/// finds the roblox STUDIO working directory. this code assumes that:
///
/// - you have supplied the directory to search, keep in mind that **there is _no_ fallback directory if it isn't supplied**
/// - roblox does not modify the way "instances" are installed. this code works as of 12/11/2023
/// - you have a match statement to see the result
///
/// returns: the full path of the working directory, if its not found, it will return `None`.
fn find_studio_exe(directory: &std::path::Path) -> Option<String> {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if fs::read_dir(&path).map(|mut dir| dir.any(|entry| entry.as_ref().map_or(false, |file| file.file_name() == "RobloxStudioBeta.exe"))).unwrap()
                {
                    return Some(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }
    }
    None
}

/// don't call this function, its only here to get the appdata path
fn init_get_local_appdata_path() -> String {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => panic!("gg {}", err) // you DO know the user is already beyond messed up if %localappdata% isn't an environment variable.
        // sure you may want to debloat/harden it but this is just too damn far bro
        // TODO: add check at the start so if this function breakks we can actually report it to the user instead of crashing instantly
    };

    local_appdata_path
}

/// this only acceps `String`s, not &str 💀
/// thank you, format!()
fn set_error(error_str: String) -> bool {
    // idfk
    let mut error_var = LAST_ERROR.lock().unwrap();
    *error_var = error_str;
    true
}

/// the actual code responsible for optimizing this.
fn apply_clientappsettings_json(client_settings: &[u8]) -> bool {
    let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path);

            if let Err(err) = fs::create_dir_all(&rblx_path) {
                set_error(format!("Error creating folder: {}", err));

                return false;
            }

            if let Err(err) = fs::write(format!("{}\\ClientAppSettings.json", rblx_path), client_settings) {
                set_error(format!("Error creating file: {}", err).to_string());

                return false;
            }
        }
        None => {
            set_error(String::from("RobloxPlayerBeta not found... do you have the game installed?"));

            return false
        }
    }

    true // we gud 👍
}

/// responsible for roblox studio ig
#[tauri::command]
fn apply_studio_config_json() -> bool {
    // use the fflags that don't affect how the game looks, since this is a developer environment
    let client_settings = include_bytes!("CAS_studio.json");
    let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    match find_studio_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path);

            if let Err(err) = fs::create_dir_all(&rblx_path) {
                set_error(format!("Error creating folder: {}", err));

                return false;
            }

            if let Err(err) = fs::write(format!("{}\\ClientAppSettings.json", rblx_path), client_settings) {
                set_error(format!("Error creating file: {}", err).to_string());

                return false;
            }
        }
        None => {
            set_error(String::from("RobloxStudioBeta not found... do you have Roblox Studio installed?"));

            return false
        }
    }

    true // we gud 👍
}

#[tauri::command]
fn get_last_error() -> String {
    // if something is holding the mutex forever we're just gonna hang here
    // :skulley:
    LAST_ERROR.lock().unwrap().to_string()
}

/// returns the app version, if this is not self-documenting code then I don't know what is.
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn optimize_perf() -> bool {
    apply_clientappsettings_json(include_bytes!("CAS_perf.json"))
}

#[tauri::command]
fn optimize_1975() -> bool {
    apply_clientappsettings_json(include_bytes!("CAS_1975.json"))
}

#[tauri::command]
fn optimize_lowspec() -> bool {
    apply_clientappsettings_json(include_bytes!("CAS_lowspec.json"))
}

#[tauri::command]
fn optimize_office() -> bool {
    apply_clientappsettings_json(include_bytes!("CAS_office.json"))
}

#[tauri::command]
fn optimize_gpu_settings() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

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

            true
        }
        None => {
            set_error(String::from("RobloxPlayerBeta not found... do you have the game installed?"));

            false
        }
    }

    // Ok(())
}

#[tauri::command]
fn unoptimize() -> bool {
    let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            match fs::remove_dir_all(format!("{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings", local_appdata_path)) {
                Ok(_) => true,
                Err(err) => {
                    set_error(format!("wtf? {}", err));

                    false
                }
            }
        }
        None => {
            set_error(String::from("RobloxPlayerBeta not found... do you have the game installed?"));

            false
        }
    }
}

/// this is where the program's entrypoint is. if this is not self-documenting code, then I don't know what is.
fn main() {
    // let args: Vec<String> = env::args().collect();

    // // assume CLI first
    // // look i know tauri has their own feature for CLIs but i like seeing the world burn
    // if args.len() > 1 {
    //     cli_attach_to_console();

    //     println!("Optimize_my_Roblos version {}", get_version());
    //     println!("Keep in mind that the updater is NOT present in CLI mode.");

    //     match args.get(1).map(|arg| arg.as_str()) {
    //         Some("-optimize") | Some("-o") => {
    //             match args.get(1).map(|arg| arg.as_str()) {
    //                 Some("perf") => {
    //                     if !optimize_perf() {
    //                         println!("Failed to apply optimizations: {}", get_last_error());
    //                     }
    //                 }
    //                 Some("nineteenseventyfive") | Some("1975") => {
    //                     if !optimize_1975() {
    //                         println!("Failed to apply optimizations: {}", get_last_error());
    //                     }
    //                 }
    //                 Some("lowspec") => {
    //                     if !optimize_lowspec() {
    //                         println!("Failed to apply optimizations: {}", get_last_error());
    //                     }
    //                 }
    //                 Some("office") => {
    //                     if !optimize_office() {
    //                         println!("Failed to apply optimizations: {}", get_last_error());
    //                     }
    //                 }
    //                 Some("studio") => {
    //                     if !apply_studio_config_json() {
    //                         println!("Failed to apply optimizations: {}", get_last_error());
    //                     }
    //                 }
    //                 _ => {
    //                     println!("parameter needed")
    //                 }
    //             }
    //         }
    //         Some("-unoptimize") | Some("-u") => {
    //             println!("removing tweaks");
    //             if !unoptimize() {
    //                 println!("Failed to remove optimizations: {}", get_last_error());
    //             }
    //         }
    //         Some("-help") | Some("-h") => {
    //             println!("=== HELP ===");
    //             println!("If this is an actual plea for help, this isn't the right place.");
    //             println!("otherwise, if you need help with CLI mode, then this is the right place");
    //             println!("-optimize -o                first switch for the optimizer");
    //             println!("              perf          optimize for performance only; no visual quality tradeoff");
    //             println!("              1975          favour (favor) maximum performance.");
    //             println!("              lowspec       default optimization profile used in GUI mode");
    //             println!("              office        if your machine is relatively new");
    //             println!("              studio        optimize Roblox Studio.");
    //             println!("-help -h                    shows the help menu, looks like you already figured this out");
    //         }
    //         _ => {
    //             println!("invalid parameter, use -help for help. please note that slash wont help you because im a terrible person");
    //         }
    //     }

    //     println!("press any key to exit");
    //     std::process::exit(420);
    // }

    // no CLI, start the GUI.
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            apply_studio_config_json,
            get_last_error,
            get_version,
            optimize_perf,
            optimize_1975,
            optimize_lowspec,
            optimize_office,
            optimize_gpu_settings,
            unoptimize
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
