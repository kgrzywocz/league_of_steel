#[repr(C)]
pub struct BackendScreenResolution {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct BackendColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

#[repr(C)]
pub struct BackendScreenAnalyzer {
    phantom_data: u8,
}
#[repr(C)]
pub struct BackendPixelRect {
    phantom_data: u8,
}

pub use crate::backend::AnalyzerHolder;

pub type FrontendAnalysisFunction = extern "C" fn(*mut AnalyzerHolder, *const BackendPixelRect);

#[no_mangle]
extern "C" {
    pub fn lollib_backend_createBackendScreenAnalyzer() -> *mut BackendScreenAnalyzer;
    pub fn lollib_backend_destroyBackendScreenAnalyzer(s: *mut BackendScreenAnalyzer);
    pub fn lollib_backend_getMode(s: *mut BackendScreenAnalyzer) -> BackendScreenResolution;
    #[allow(improper_ctypes)]
    pub fn lollib_backend_analyzeScreenshot(
        s: *mut BackendScreenAnalyzer,
        analyzer_holder: *mut AnalyzerHolder,
        analyzeFunction: FrontendAnalysisFunction,
    );

    pub fn lollib_backend_pixelRect_getHight(rect: *const BackendPixelRect) -> u32;
    pub fn lollib_backend_pixelRect_getWidth(rect: *const BackendPixelRect) -> u32;
    pub fn lollib_backend_pixelRect_getColor(
        rect: *const BackendPixelRect,
        x: u32,
        y: u32,
    ) -> BackendColor;

    pub fn lollib_backend_is_process_running(exe_name: *const std::os::raw::c_char) -> i32;
    pub fn lollib_backend_get_process_exe_path(
        exe_name: *const std::os::raw::c_char,
        output: *mut std::os::raw::c_char,
        output_length: libc::size_t,
    );
}
