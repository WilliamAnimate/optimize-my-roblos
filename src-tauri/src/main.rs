// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate terrible_roblox_optimizer;

use std::{env, sync::Mutex};

// #[cfg(windows)] use winreg::{enums::*, RegKey};

use terrible_roblox_optimizer::{optimize_directory_specific_file, unoptimize_directory};
use lazy_static::lazy_static;

use backtrace::Backtrace;

lazy_static!(
    static ref LAST_ERROR: Mutex<String> = Mutex::new(String::from("No error details were set."));
);

/// this only acceps `String`s, not &str 💀
///
/// thank you, format!()
fn set_error(error_str: String) {
    let bt = Backtrace::new();
    println!("we've got hostiles:\n{:?}", bt);

    *LAST_ERROR.lock().unwrap() = error_str.clone();
}

/// responsible for roblox studio ig
#[tauri::command]
fn apply_studio_config_json() -> bool {
    match optimize_directory_specific_file("RobloxStudioBeta.exe", include_bytes!("CAS_studio.json")) {
        Ok(_) => return true,
        Err(err) => set_error(err.to_string()),
    }

    false
}

#[tauri::command]
fn get_last_error() -> String {
    // if something is holding the mutex forever we're just gonna hang here
    LAST_ERROR.lock().unwrap().to_string()
}

/// returns the app version, if this is not self-documenting code then I don't know what is.
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn optimize_wrapper(file: &str, client_settings: &[u8]) -> bool {
    match optimize_directory_specific_file(file, client_settings) {
        Ok(_) => return true,
        Err(err) => {
            set_error(err.to_string());
            false
        }
    }
}

#[tauri::command]
fn optimize_perf() -> bool {
    optimize_wrapper("RobloxPlayerBeta.exe", include_bytes!("CAS_perf.json"))
}

#[tauri::command]
fn optimize_1975() -> bool {
    optimize_wrapper("RobloxPlayerBeta.exe", include_bytes!("CAS_1975.json"))
}

#[tauri::command]
fn optimize_lowspec() -> bool {
    optimize_wrapper("RobloxPlayerBeta.exe", include_bytes!("CAS_lowspec.json"))
}

#[tauri::command]
fn optimize_office() -> bool {
    optimize_wrapper("RobloxPlayerBeta.exe", include_bytes!("CAS_office.json"))
}

#[tauri::command]
fn optimize_gpu_settings() -> bool {
    todo!("make this code work again, like putting it into the crate?");
    // #[cfg(windows)] {
    //     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    //     // let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    //     let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    //     match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
    //         Some(result_folder_name) => {
    //             let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\RobloxPlayerBeta.exe", local_appdata_path);

    //             let path_dx11 = Path::new("Software\\Microsoft\\DirectX\\UserGpuPreferences");
    //             let path_app_compat_flags = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers");
    //             // let perf_options = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RobloxPlayerBeta.exe\\PerfOptions");

    //             let Ok((key_dx11, _)) = hkcu.create_subkey(&path_dx11) else {panic!("gg")};
    //             let Ok((key_app_compat_flags, _)) = hkcu.create_subkey(&path_app_compat_flags) else {panic!("gg")};
    //             // let (key_perf_options, _) = hklm.create_subkey(&perf_options);

    //             // note: clone the rblx_path so it doesn't mess things up
    //             let _ = key_dx11.set_value(rblx_path.clone(), &"GpuPreference=2;");
    //             let _ = key_app_compat_flags.set_value(rblx_path, &"~ DISABLEDXMAXIMIZEDWINDOWEDMODE");
    //             // key_perf_options.set_value("CpuPriorityClass", &"3");

    //             true
    //         }
    //         None => {
    //             set_error(String::from("RobloxPlayerBeta not found... do you have the game installed?"));

    //             false
    //         }
    //     }
    // }
    // #[cfg(not(windows))] {
    //     unimplemented!("you can't change registry in linux because linux doesn't have registry, silly.");
    // }
}

/// this code will unoptimize both roblox AND studio.
#[tauri::command]
fn unoptimize() -> bool {
    match unoptimize_directory("RobloxPlayerBeta.exe") {
    // && unoptimize_directory("RobloxStudioBeta.exe") {
        Ok(_) => {
            match unoptimize_directory("RobloxStudioBeta.exe") {
                Ok(_) => return true,
                Err(err) => set_error(format!("Can't find Studio, do you have it installed? {}", err)),
            }
        },
        Err(err) => set_error(format!("Can't find Roblox, do you have it installed? {}", err)),
    }

    false
}

/// this is where the program's entrypoint is. if this is not self-documenting code, then I don't know what is.
fn main() {
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
        .expect(""); // empty message to save like idk a few bytes or some
                         // if it even saves anything in the first place.
}
