use crate::bindings::*;


fn dummy_analyze_function(_rect: PixelRect) -> LolStats {
    LolStats {
        health: 0,
        mana: 0,
        hit: 0,
    }
}

pub type AnalyzeFunction = fn(PixelRect) -> LolStats;
static mut ANALYZE_FUNCTION : AnalyzeFunction = dummy_analyze_function;

pub struct Backend {
    backend_screen_analyzer: *mut BackendScreenAnalyzer,
}
impl Backend {
    pub fn new(analyze_function: AnalyzeFunction) -> Self {
        unsafe{ANALYZE_FUNCTION = analyze_function};
        Self::new_low_lewel(Self::frontend_analysis_function)
    }

    extern "C" fn frontend_analysis_function(
        backend_pixel_rect: *const BackendPixelRect,
    ) -> LolStats {
        let pixel_rect = PixelRect::new(backend_pixel_rect);
        unsafe { ANALYZE_FUNCTION(pixel_rect) }
    }

    fn new_low_lewel(analyze_function: FrontendAnalysisFunction) -> Self {
        unsafe {
            let backend_screen_analyzer =
                lollib_backend_createBackendScreenAnalyzer(analyze_function);
            if std::ptr::null() == backend_screen_analyzer {
                panic!("lollib_backend not correctly initialized!");
            }
            Self {
                backend_screen_analyzer,
            }
        }
    }

    pub fn get_mode(&self) -> BackendScreenResolution {
        unsafe { lollib_backend_getMode(self.backend_screen_analyzer) }
    }
    pub fn has_mode_changed(&self) -> bool {
        unsafe { lollib_backend_hasModeChanged(self.backend_screen_analyzer) !=0  }
    }
    pub fn set_capture_rect(&mut self, capture_rect: &BackendCaptureRect) {
        unsafe {
            lollib_backend_setCaptureRect(self.backend_screen_analyzer, capture_rect);
        }
    }
    pub fn analyze_screenshot(&self) -> LolStats {
        unsafe { lollib_backend_analyzeScreenshot(self.backend_screen_analyzer) }
    }
}
impl Drop for Backend {
    fn drop(&mut self) {
        unsafe { lollib_backend_destroyBackendScreenAnalyzer(self.backend_screen_analyzer) };
    }
}

pub struct PixelRect {
    rect: *const BackendPixelRect,
}

impl PixelRect {
    fn new(rect: *const BackendPixelRect) -> Self {
        if std::ptr::null() == rect {
            panic!("lollib_backend BackendPixelRect not correctly initialized!");
        }
        Self { rect }
    }

    pub fn get_hight(&self) -> i32 {
        unsafe { lollib_backend_pixelRect_getHight(self.rect) }
    }
    pub fn get_width(&self) -> i32 {
        unsafe { lollib_backend_pixelRect_getWidth(self.rect) }
    }
    pub fn get_color(&self, row: i32, column: i32) -> Color {
        Color::from_backend(
            unsafe { lollib_backend_pixelRect_getColor(self.rect, row, column) }
        )
    }
}

pub struct Color{
    color : BackendColor
}

impl Color{
    fn from_backend(backend_color:BackendColor) ->Self{
        Self{color: backend_color}
    }
    pub fn is_red(&self) ->bool
    {
      return self.color.r as i32 > self.color.b  as i32 + 50 && 
      self.color.r as i32 >self.color.g as i32 + 50;
    }
    pub fn is_green(&self) ->bool
    {
      return self.color.g as i32 > self.color.b as i32 + 50 && 
      self.color.g as i32 > self.color.r as i32 + 50;
    }
    pub fn is_blue(&self) ->bool
    {
      return self.color.b as i32 > self.color.g as i32 + 50 && 
      self.color.b as i32 >self.color.r as i32  + 50;
    }
    pub fn is_yellow(&self) ->bool
    {
      return self.color.r as i32 > self.color.b as i32 + 50 && 
      self.color.g as i32 > self.color.b as i32 + 50 && (self.color.r as i8 -self.color.g as i8).abs() < 50;
    }
}
