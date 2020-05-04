#[repr(C)]
pub struct LolStats {
    pub health: u8,
    pub mana: u8,
    pub hit: u8,
}

#[repr(C)]
pub struct BackendCaptureRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

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
pub struct BackendScreenAnalyzer{
    phantom_data : u8,
}
#[repr(C)]
pub struct BackendPixelRect{
    phantom_data : u8,
}

pub type FrontendAnalysisFunction = extern "C" fn(*const BackendPixelRect) -> LolStats;

#[no_mangle]
extern "C" {
    pub fn lollib_backend_createBackendScreenAnalyzer(analyzeFunction: FrontendAnalysisFunction) -> *mut BackendScreenAnalyzer;
    pub fn lollib_backend_destroyBackendScreenAnalyzer(s: *mut BackendScreenAnalyzer);
    pub fn lollib_backend_getMode(s: *mut BackendScreenAnalyzer) -> BackendScreenResolution;
    pub fn lollib_backend_hasModeChanged(s: *mut BackendScreenAnalyzer) -> i32;
    pub fn lollib_backend_setCaptureRect(s: *mut BackendScreenAnalyzer, captureRect: *const BackendCaptureRect);
    pub fn lollib_backend_analyzeScreenshot(s: *mut BackendScreenAnalyzer) -> LolStats;

    pub fn lollib_backend_pixelRect_getHight(rect: *const BackendPixelRect) -> i32;
    pub fn lollib_backend_pixelRect_getWidth(rect: *const BackendPixelRect) -> i32;
    pub fn lollib_backend_pixelRect_getColor(rect: *const BackendPixelRect, row: i32, column: i32) -> BackendColor;
}
