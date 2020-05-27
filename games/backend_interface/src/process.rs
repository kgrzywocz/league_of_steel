use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::bindings::*;

pub fn is_process_running(exe_name: &str) -> bool {
    let exe_name_cstr = CString::new(exe_name)
        .expect("Failed to create C-String")
        .into_raw();

    let res = unsafe { lollib_backend_is_process_running(exe_name_cstr) };

    unsafe {
        let _ = CString::from_raw(exe_name_cstr);
    };
    res != 0
}

pub fn get_process_exe_path(exe_name: &str) -> String {
    let exe_name_cstr = std::ffi::CString::new(exe_name)
        .expect("Failed to create C-String")
        .into_raw();

    let mut buf = [0 as c_char; 255];
    let c_str: &CStr;
    unsafe {
        lollib_backend_get_process_exe_path(exe_name_cstr, buf.as_mut_ptr(), buf.len());
        let _ = CString::from_raw(exe_name_cstr);
        c_str = CStr::from_ptr(buf.as_ptr());
    };
    c_str.to_str().expect("Invalid path formatting").to_owned()
}
