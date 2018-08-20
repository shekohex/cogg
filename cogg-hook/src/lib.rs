#![cfg(windows)]
#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

use failure::Error;
use std::io::Error as IoError;
use winapi::shared::minwindef as wintype;
use winapi::um::winuser;
use std::fs::File;
use std::io::prelude::*;
use winapi::um::consoleapi;

#[allow(dead_code)]
type MyResult<T> = std::result::Result<T, Error>;

/// Entry point which will be called by the system once the DLL has been loaded
/// in the target process. Declaring this function is optional.
///
/// # Safety
///
/// What you can safely do inside here is very limited, see the Microsoft documentation
/// about "DllMain". Rust also doesn't officially support a "life before main()",
/// though it is unclear what that that means exactly for DllMain.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: wintype::HINSTANCE,
    call_reason: wintype::DWORD,
    reserved: wintype::LPVOID,
) -> wintype::BOOL {
    const DLL_PROCESS_ATTACH: wintype::DWORD = 1;
    const DLL_PROCESS_DETACH: wintype::DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => demo_init(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    wintype::TRUE
}

fn demo_init() {
    unsafe { consoleapi::AllocConsole() };
    println!("Hello, world!");
    msg_box("WOW, Msgbox Working :O ", winuser::MB_OK).unwrap();
    // let mut file = File::create("gglogs.txt").unwrap();
    // file.write_all(b"Test Logging!").unwrap();
}

fn msg_box(msg: &str, mtype: wintype::UINT) -> Result<i32, IoError> {
use std::os::windows::prelude::*;
    use std::ffi::OsStr;
    use std::iter::once;
    use std::ptr::null_mut;
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe { winuser::MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), mtype) };

    if ret == 0 {
        Err(IoError::last_os_error())
    } else {
        Ok(ret)
    }
}
