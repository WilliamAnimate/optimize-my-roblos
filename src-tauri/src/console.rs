const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;

extern "system" {
    fn AttachConsole(dw_process_id: u32) -> i32;
    fn FreeConsole() -> i32;
    // fn AllocConsole() -> i32;
    // fn ShowWindow(handle: *mut std::ffi::c_void, n_cmd_show: i32) -> i32;
    // fn GetConsoleWindow() -> *mut std::ffi::c_void;
}

// /// creates a new console to print debug output to, in the case of `#![windows_subsystem = "windows"])` being on
// pub fn show_console() { unsafe {
// 	AllocConsole();
// 	let console_handle = GetConsoleWindow();

// 	ShowWindow(console_handle, 5); // SW_SHOWNORMAL
// }}

/// attaches to the current console in the case of `#![windows_subsystem = "windows"])` being on
///
/// this works differently than show_console() because this attaches, not creating a new one
///
/// # IF YOU CALL THIS FUNCTION, PLEASE CALL console::cli_detach_from_console() TO SHOW THE SYSTEM'S DEFAULT $s$g PROMPT!
pub fn cli_attach_to_console() { unsafe {
	AttachConsole(ATTACH_PARENT_PROCESS);
}}

/// detaches from the current console
/// # use only when exiting
pub fn cli_detach_from_console() { unsafe {
    FreeConsole();
}}
