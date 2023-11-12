//
// Quick standalone app for optimizing the way Windows schedules and handles the RobloxPlayerBeta.exe process
// - Sets gpu to gpu2, which is usually your dedicated one
// - Disables fullscreen (as in exclusive-fullscreen, f11) optimizations, which reduces input lag (prevents DWM from taking control over the display, letting the game do whatevertheheckitwantswiththedisplay)
// - Sets the priority to HIGH_PRIORITY_CLASS (high) on launch, which adjusts how the NT kernel schedules the processes; makes Roblox have more time on the CPU, maximizing performance
// that may or may not cause input lag, as input is *probably* handled by csrss, which runs at NORMAL_PRIORITY_CLASS (normal)

// reg add "HKCU\Software\Microsoft\DirectX\UserGpuPreferences" /v "%rblx_path%" /t Reg_SZ /d "GpuPreference=2;" /f
// reg add "HKCU\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers" /v "%rblx_path%" /t Reg_SZ /d "~ DISABLEDXMAXIMIZEDWINDOWEDMODE" /f
// reg add "HKLM\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\RobloxPlayerBeta.exe\PerfOptions" /v "CpuPriorityClass" /t Reg_DWORD /d "3" /f

use std::fs;
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("This code has no error handling! Press any key to execute.");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let local_appdata_path = local_appdata();

    match find_roblox_exe(&std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!("{}\\Roblox\\Versions\\{result_folder_name}\\RobloxPlayerBeta.exe", local_appdata_path);

            let path_dx11 = Path::new("Software\\Microsoft\\DirectX\\UserGpuPreferences");
            let path_app_compat_flags = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers");
            let perf_options = Path::new("Software\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RobloxPlayerBeta.exe\\PerfOptions");

            let (key_dx11, _) = hkcu.create_subkey(&path_dx11)?;
            let (key_app_compat_flags, _) = hkcu.create_subkey(&path_app_compat_flags)?;
            let (key_perf_options, _) = hklm.create_subkey(&perf_options)?;

            // note: clone the rblx_path so it doesn't mess things up
            key_dx11.set_value(rblx_path.clone(), &"GpuPreference=2;")?;
            key_app_compat_flags.set_value(rblx_path, &"~ DISABLEDXMAXIMIZEDWINDOWEDMODE")?;
            key_perf_options.set_value("CpuPriorityClass", &"3")?;
        }
        None => eprintln!("RobloxPlayerBeta not found... do you have the game installed?"),
    }

    Ok(())
}

fn local_appdata() -> String {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get APPDATA path: {}", err);
            "err".to_string() // i hate rust
        }
    };

    local_appdata_path
}

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