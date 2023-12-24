use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs;

lazy_static! {
    static ref LOCALAPPDATA_PATH: Mutex<String> = Mutex::new(get_localappdata_path());
}

// checklist:
// find executable in directory
// unoptimize directory
// optimize a directory
// TODO :
// documentation

/// don't call this function, its only here to get the appdata path
fn get_localappdata_path() -> String {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => panic!("what? {}", err),
    };

    local_appdata_path
}

/// finds the roblox working directory. this code assumes that:
///
/// - you have supplied the directory to search, keep in mind that **there is _no_ fallback directory if it isn't supplied**
/// - roblox does not modify the way "instances" are installed. this code works as of 11/11/2023
/// - you have a match statement to see the result
///
/// returns the full path of the working directory, if its not found, it will return `None`.
///
/// # Example
///
/// ```rust
/// let file_to_find = "RobloxPlayerBeta.exe";
/// let directory = std::env::current_dir().unwrap().join(format!("{local_appdata_path}\\Roblox\\Versions"));
/// let result = find_executable_in_directory(file_to_find, &directory);
/// println!("{:?}", result);
/// ```
// TODO: fix indentation hell
fn find_executable_in_directory(file_to_find: &str, directory: &std::path::Path) -> Option<String> {
    dbg!(&file_to_find);

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if fs::read_dir(&path)
                    .map(|mut dir| {
                        dir.any(|entry| {
                            entry
                                .as_ref()
                                .map_or(false, |file| file.file_name() == file_to_find)
                        })
                    })
                    .unwrap()
                {
                    return Some(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

/// removes all ClientSettings tweaks.
///
/// removes tweaks on a directory if it has the file with the parameter `flag_file`.
///
/// very clear explaination, i know
pub fn unoptimize_directory(flag_file: &str) -> Result<(), std::io::Error> {
    let local_appdata_path = match std::env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(err) => panic!("wheres your %localappdata%? {}", err),
    };

    match find_executable_in_directory(
        flag_file,
        &std::env::current_dir()
            .unwrap()
            .join(format!("{local_appdata_path}\\Roblox\\Versions")),
    ) {
        Some(result_folder_name) => {
            match fs::remove_dir_all(format!(
                "{local_appdata_path}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings"
            )) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        None => {
            // FIXME: is there another way to do this? i literally asked chatgpt lmao
            Err(*Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from("RobloxPlayerBeta.exe not found! Do you have the game installed?"),
            )))
        }
    }
}

/// flag_file: the file to look for
/// client_settings: include_bytes! variable. this should be the ClientSettings.json file.
pub fn optimize_directory_specific_file(flag_file: &str, client_settings: &[u8; 0]) -> Result<(), std::io::Error> {
    // use the fflags that don't affect how the game looks, since this is a developer environment
    // let client_settings: &[u8; 0] = include_bytes!("CAS_studio.json");
    let local_appdata_path = LOCALAPPDATA_PATH.lock().unwrap().to_string();

    match find_executable_in_directory(flag_file, &std::env::current_dir().unwrap().join(format!("{}\\Roblox\\Versions", local_appdata_path))) {
        Some(result_folder_name) => {
            let rblx_path = format!(
                "{}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings",
                local_appdata_path
            );

            if let Err(err) = fs::create_dir_all(&rblx_path) {
                return Err(err);
            }

            if let Err(err) = fs::write(
                format!("{}\\ClientAppSettings.json", rblx_path),
                client_settings,
            ) {
                return Err(err);
            }
        }
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, String::from("Roblox Not Found")));
        }
    }

    Ok(())
}

//#region randomshit
/*
allow me to explain what i tried to do here
so basically i tried to return custom error types
yewah very nice
TODO: delete this "code" someday
*/

// fn give_me_an_err_varient(text: &str) -> Result<std::io::Error, _> {
//     Err(*Box::new(std::io::Error::new(std::io::ErrorKind::Other, String::from(text))))
// }

// GiveErr is so stupid, imagine if there was just one macro that would do this.
// #[derive(Debug)]
// struct GiveErr {
//     message: String,
// }

// impl fmt::Display for GiveErr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// impl From<&str> for GiveErr {
//     fn from(message: &str) -> Self {
//         GiveErr {
//             message: String::from(message),
//             // message: (*Box::new(std::io::Error::new(std::io::ErrorKind::Other, String::from(message)))).to_string()
//         }
//     }
// }
// // fn give_me_an_err_variant(text: &str) -> Result<(), GiveErr> {
// //     Err(GiveErr::from(text))
// // }

// struct MyError {
//     message: String,
// }

// impl From<&str> for MyError {
//     fn from(item: &str) -> Self {
//         MyError {
//             message: item.to_string(),
//         }
//     }
// }
//#endregion randomshit

#[cfg(test)]
mod tests {
    use crate::{find_executable_in_directory, unoptimize_directory};

    // #[test]
    // fn get_error() {
    //     match MyError::from("this") {
    //         Ok(_) => panic!("wtf"),
    //         Err(err) => println!("it works! {:?}", err),
    //     }
    // }

    #[test]
    fn identify_roblox_directory() {
        // you can't use assert_eq here, the directory keeps changing on every roblox update.
        // use your own judgement.
        // println!("you can't use assert_eq!() here, the directory keeps changing on every roblox update.\nuse your own judgement.\r\n\r\n");

        let local_appdata_path = match std::env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(err) => panic!("wheres your %localappdata%? {}", err),
        };

        let testresult = find_executable_in_directory(
            "RobloxPlayerBeta.exe",
            &std::env::current_dir()
                .unwrap()
                .join(format!("{}\\Roblox\\Versions", local_appdata_path)),
        );

        dbg!(&testresult);
        match testresult {
            Some(ref r) => {
                println!("nice. once again, the value is {:?}", r);
            }
            None => {
                println!("did not find");
            }
        }
    }
    #[test]
    fn unoptimize_roblox() {
        let local_appdata_path = match std::env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(err) => panic!("wheres your %localappdata%? {}", err),
        };

        match find_executable_in_directory(
            "RobloxPlayerBeta.exe",
            &std::env::current_dir()
                .unwrap()
                .join(format!("{local_appdata_path}\\Roblox\\Versions")),
        ) {
            Some(result_folder_name) => {
                let directory_path = format!(
                    "{local_appdata_path}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings"
                );
                dbg!(&directory_path);
                if let Ok(_) = std::fs::metadata(directory_path) {
                    println!("PREDELETE: the directory exists");
                } else {
                    println!("PREDELETE: the directory DOES NOT exist.");
                }
            }
            None => {
                panic!("aaaaaaaaaaa");
            }
        }

        println!(
            "Unoptimize: {:?}",
            unoptimize_directory("RobloxPlayerBeta.exe")
        );

        match find_executable_in_directory(
            "RobloxPlayerBeta.exe",
            &std::env::current_dir()
                .unwrap()
                .join(format!("{local_appdata_path}\\Roblox\\Versions")),
        ) {
            Some(result_folder_name) => {
                let directory_path = format!(
                    "{local_appdata_path}\\Roblox\\Versions\\{result_folder_name}\\ClientSettings"
                );
                dbg!(&directory_path);
                if let Ok(_) = std::fs::metadata(directory_path) {
                    panic!("directory deletion failed");
                } else {
                    println!("The directory has been deleted");
                }
            }
            None => {
                panic!("aaaaaaaaaaa");
            }
        }
    }
}
