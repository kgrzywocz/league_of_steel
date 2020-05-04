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

struct Backend{
    backend_screen_analyzer: *mut BackendScreenAnalyzer,
}
impl Backend{
    fn new(analyze_function: FrontendAnalysisFunction)->Self{
        unsafe{
            let backend_screen_analyzer=lollib_backend_createBackendScreenAnalyzer(analyze_function);
            if std::ptr::null() == backend_screen_analyzer
            {
                panic!("lollib_backend not correctly initialized!");
            }
            Self{backend_screen_analyzer}
        }
    }

    pub fn get_mode(&self) -> BackendScreenResolution{
        unsafe{
            lollib_backend_getMode(self.backend_screen_analyzer)
        }
    }
    pub fn has_mode_changed(&self) -> i32{
        unsafe{
            lollib_backend_hasModeChanged(self.backend_screen_analyzer)
        }
    }
    pub fn set_capture_rect(&self, capture_rect: *const BackendCaptureRect){
        unsafe{
            lollib_backend_setCaptureRect(self.backend_screen_analyzer, capture_rect);
        }
    }
    pub fn analyze_screenshot(&self) -> LolStats{
        unsafe{
            lollib_backend_analyzeScreenshot(self.backend_screen_analyzer)
        }
    }
}
impl Drop for Backend{
    fn drop(&mut self){
        unsafe{
            lollib_backend_destroyBackendScreenAnalyzer(self.backend_screen_analyzer)
        };
    }
}

struct PixelRectWraper
{
    rect: *const BackendPixelRect,
}

impl PixelRectWraper{
    fn new(rect: *const BackendPixelRect) ->Self
    {
        if std::ptr::null() == rect
        {
            panic!("lollib_backend BackendPixelRect not correctly initialized!");
        }
        Self{rect}
    }

    pub fn get_hight(&self) -> i32{
        unsafe{
            lollib_backend_pixelRect_getHight(self.rect)
        }
    }
    pub fn get_width(&self) -> i32{
        unsafe{
            lollib_backend_pixelRect_getWidth(self.rect)
        }
    }
    pub fn get_color(&self, row: i32, column: i32) -> BackendColor{
        unsafe{
            lollib_backend_pixelRect_getColor(self.rect, row, column)
        }
    }
}