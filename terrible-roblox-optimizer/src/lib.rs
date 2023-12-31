use std::fs;
#[cfg(unix)] use home::home_dir; // to get the home dir in unix. you can't just use ~ for some reason
#[cfg(windows)] use winreg::{enums::*, RegKey}; // for GPU settings

/// Optimizes vinegar's config.
///
/// This code returns a Result, so it may be preferrable to put this code in a `match`, `.result` or `?`
///
/// ```rust
/// let vinegar_config: &[u8] = include_bytes!("vinegar.toml"); // replace this with the actual toml file of your vinegar config.
///                                                             // it should be located inside the `src/` directory
///
/// optimize_linux_vinegar(&vinegar_config)?;
/// ```
pub fn optimize_linux_vinegar(vinegar_config: &[u8]) -> Result<(), std::io::Error> {
    #[cfg(unix)] {
        // OKAY, linux is unix but not all unix is linux
        // but im going to take an assumption that this *is* linux lmao

        // vinegar path for me is /home/william/.config/vinegar/config.toml, which can translate to
        // ~/.config/vinegar/config.toml
        // TODO: test for other platforms, since i installed vinegar with `yay`.

        // const VINEGAR_CONFIG: &[u8] = include_bytes!("vinegar_config.toml");

        let vinegar_path = home_dir().expect("failed to get home directory").join(".config/vinegar/config.toml");

        // if let Err(err) = fs::remove_file(&vinegar_path) {
        //     set_error(format!("can't remove previous vinegar config... try running me as superuser? {}", err));
        //
        //     return false;
        // }

        match fs::write(vinegar_path, vinegar_config) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }

        // Ok(())
    }

    #[cfg(windows)] {
        unimplemented!("you don't need a compatibility layer like vinegar on windows, silly");
    }
}

/// Attempts to do a bit of registry tweaking to optimize roblox for windows.
///
/// # Panics
/// This code will panic (call unimplemented!) if called on anything that isn't windows.
///
/// # Examples
/// ```rust
/// // match because this code returns a Result value.
/// match windows_optimize_gpu_settings() {
///     Ok(_) => println!("it worked"),
///     Err(err) => eprintln!("it didn't work: {}", err),
/// }
/// ```
pub fn windows_optimize_gpu_settings() -> Result<(), std::io::Error> {
    #[cfg(windows)] {
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

                Ok(())
            }
            None => {
                set_error(String::from("RobloxPlayerBeta not found... do you have the game installed?"));
                Err::<T, String>(format!("RobloxPlayerBeta not found... do you have the game installed? {}", err));
            }
        }
    }
    // anything that ISN'T windows
    #[cfg(not(windows))] {
        unimplemented!("the registry you're telling me to write to is only on windows (and reactos...), silly.");
    }
}

/// unoptimizes a specific directory based on the `executable_name` parameter
/// # Panics
/// This code will panic if running under anything other than windows
pub fn unoptimize_roblox_clientsettings(executable_name: &str, path: &str) -> Result<(), std::io::Error> {
    // let local_appdata_path: String = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    #[cfg(windows)] {
        match find_directory_containing_executable(&executable_name, &std::env::current_dir().unwrap().join(path)) {
            Some(result_folder_name) => {
                match fs::remove_dir_all(format!("{result_folder_name}")) {
                    Ok(_) => return Ok(()),
                    Err(err) => return Err(err),
                };
            }
            None => {
                // Err(*Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{executable_name} not found."))))
                todo!("add error handling here wtf?");
            }
        }
    }
    #[cfg(not(windows))] {
        unimplemented!("unsupported platform, now heres the stuff you called here so it'd stop showing errors: {executable_name}, {path}");
    }
}
