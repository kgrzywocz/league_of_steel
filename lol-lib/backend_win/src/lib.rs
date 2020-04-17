extern crate libc;

extern "C" {
    fn lollib_is_lol_running() -> i32;
    fn lollib_lol_exe_path(output: *mut u8, output_length: libc::size_t);
}

pub fn is_lol_running() -> bool {
    let res = unsafe { lollib_is_lol_running() };
    res != 0
}

pub fn lol_exe_path() -> String {
    let mut buf = [0u8; 255];
    unsafe { lollib_lol_exe_path(buf.as_mut_ptr(), buf.len()) };
    String::from_utf8(buf.to_vec()).expect("Invalid path formating")
}
