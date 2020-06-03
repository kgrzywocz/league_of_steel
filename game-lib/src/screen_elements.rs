use backend_interface::{Color, PixelRect};

pub struct Bar {
    height: u32,
    begin: u32,
    end: u32,
}
impl Bar {
    pub fn new() -> Self {
        Self {
            height: 0,
            begin: 0,
            end: 0,
        }
    }
    pub fn set_position(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_range(&mut self, range: (u32, u32)) {
        self.begin = range.0;
        self.end = range.1;
    }
    pub fn analyze_pixels<F>(&self, pixels: &PixelRect, predicate: F, start_from_percent: u32) -> u8
    where
        F: Fn(&Color) -> bool,
    {
        let max_gap = 20;

        let row_len = self.end - self.begin;
        let offset = start_from_percent * row_len / 100 + self.begin;

        let mut last = offset;
        for x in offset..self.end {
            if predicate(&pixels.get_color(x, self.height)) {
                last = x;
            } else if x > last + max_gap {
                break;
            }
        }

        return ((100 * (last - self.begin + 1) / row_len) - start_from_percent) as u8;
    }
}
