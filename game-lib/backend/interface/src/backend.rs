use crate::bindings::*;

pub struct AnalyzerHolder<'a> {
    pub pixel_rect_analyzer: &'a mut dyn PixelRectAnalyzer,
}

pub trait PixelRectAnalyzer {
    fn analyze_function(&mut self, rect: &PixelRect);
}

pub struct Backend {
    backend_screen_analyzer: *mut BackendScreenAnalyzer,
}
impl Backend {
    pub fn new() -> Self {
        let backend_screen_analyzer = unsafe {
            let backend_screen_analyzer = lollib_backend_createBackendScreenAnalyzer();
            if std::ptr::null() == backend_screen_analyzer {
                panic!("lollib_backend not correctly initialized!");
            }
            backend_screen_analyzer
        };
        Self {
            backend_screen_analyzer,
        }
    }

    pub fn get_mode(&self) -> BackendScreenResolution {
        unsafe { lollib_backend_getMode(self.backend_screen_analyzer) }
    }

    pub fn analyze_screenshot(&self, pixel_rect_analyzer: &mut dyn PixelRectAnalyzer) {
        let mut analyzer_holder = AnalyzerHolder {
            pixel_rect_analyzer,
        };

        unsafe {
            lollib_backend_analyzeScreenshot(
                self.backend_screen_analyzer,
                &mut analyzer_holder,
                Self::frontend_analysis_function,
            )
        }
    }

    extern "C" fn frontend_analysis_function(
        analyzer_holder_raw_ptr: *mut AnalyzerHolder,
        backend_pixel_rect: *const BackendPixelRect,
    ) {
        let analyzer_holder = unsafe {
            analyzer_holder_raw_ptr
                .as_mut()
                .expect("analyzer_holder_raw_ptr is NULL")
        };

        let pixel_rect = PixelRect::new(backend_pixel_rect);
        analyzer_holder
            .pixel_rect_analyzer
            .analyze_function(&pixel_rect);
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

    pub fn get_hight(&self) -> u32 {
        unsafe { lollib_backend_pixelRect_getHight(self.rect) }
    }
    pub fn get_width(&self) -> u32 {
        unsafe { lollib_backend_pixelRect_getWidth(self.rect) }
    }
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        Color::from_backend(unsafe { lollib_backend_pixelRect_getColor(self.rect, x, y) })
    }
}

pub struct Color {
    color: BackendColor,
}

impl Color {
    fn from_backend(backend_color: BackendColor) -> Self {
        Self {
            color: backend_color,
        }
    }
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        return (self.color.r, self.color.g, self.color.b);
    }
    pub fn is_red(&self) -> bool {
        return is_much_higher(self.color.r, self.color.b)
            && is_much_higher(self.color.r, self.color.g);
    }
    pub fn is_green(&self) -> bool {
        return is_much_higher(self.color.g, self.color.b)
            && is_much_higher(self.color.g, self.color.r);
    }
    pub fn is_blue(&self) -> bool {
        return is_much_higher(self.color.b, self.color.g)
            && is_much_higher(self.color.b, self.color.r);
    }
    pub fn is_yellow(&self) -> bool {
        return is_much_higher(self.color.r, self.color.b)
            && is_much_higher(self.color.g, self.color.b)
            && (self.color.r as i32 - self.color.g as i32).abs() < 50;
    }
}
fn is_much_higher(a: u8, b: u8) -> bool {
    ((a as i32 > b as i32 + 22) || (0.7 * a as f32 > b as f32))
}
