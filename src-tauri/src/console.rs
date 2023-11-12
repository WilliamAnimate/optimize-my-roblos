use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;

extern "system" {
    fn AttachConsole(dw_process_id: u32) -> i32;
    fn AllocConsole() -> i32;
    // fn FreeConsole() -> i32;
    fn ShowWindow(handle: *mut std::ffi::c_void, n_cmd_show: i32) -> i32;
    fn GetConsoleWindow() -> *mut std::ffi::c_void;
    fn SetConsoleTitleW(lp_console_title: *const u16) -> i32;
}

pub fn show_console() { unsafe {
	let console_title: Vec<u16> = OsStr::new("Optimize_my_Roblos debug output").encode_wide().chain(Some(0).into_iter()).collect();
	AllocConsole();
	SetConsoleTitleW(console_title.as_ptr());
	let console_handle = GetConsoleWindow();

	ShowWindow(console_handle, 5); // SW_SHOWNORMAL
}}

// pub fn hide_console() { unsafe {
// 	let console_handle = GetConsoleWindow();
// 	ShowWindow(console_handle, 0); // SW_HIDE

// 	FreeConsole();
// }}

/// attaches to the current console in the case of `#![windows_subsystem = "windows"])` being on
///
/// this works differently than show_console() because this attaches, not creating a new one
///
/// please note that when the program's execution ends, the default terminal $s$g will not be shown without user input. FIXME: that
///
/// and please remember that this is a work in progress, don't use this 💀
pub fn cli_attach_to_console() { unsafe {
	AttachConsole(ATTACH_PARENT_PROCESS);

	println!(""); // print nothing to fix something
				  // go figure it out yourself, future me
}}
