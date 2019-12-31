extern crate winapi;

use winapi::shared::minwindef::TRUE;
use winapi::um::wingdi::DEVMODEW;
use winapi::um::winuser::{ChangeDisplaySettingsW, EnumDisplaySettingsW};
use winapi::um::winuser::{CDS_FULLSCREEN, DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS};

pub fn set_resolution(width: u32, height: u32) -> bool {
    let mut devmode: DEVMODEW = Default::default();

    unsafe {
        if EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode) == TRUE {
            devmode.dmPelsWidth = width;
            devmode.dmPelsHeight = height;
            return ChangeDisplaySettingsW(&mut devmode, CDS_FULLSCREEN) == DISP_CHANGE_SUCCESSFUL;
        }
    }
    return false;
}

pub fn refocus() {
    unsafe {
        winapi::um::winuser::SetForegroundWindow(winapi::um::winuser::FindWindowA(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ));
    }
}
