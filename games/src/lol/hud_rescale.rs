pub struct Rescaler {
    screen_width: u32,
    screen_height: u32,
    hud_scale: f32,
    ratio: f64,
    ratio_normalized: f64,
}
impl Rescaler {
    pub fn new(screen_width: u32, screen_height: u32, hud_scale: f32) -> Self {
        let hud_scale = 0.35 * hud_scale + 0.65;
        let ratio = screen_width as f64 / screen_height as f64;
        let ratio_normalized = ratio / (1920.0 / 1080.0);
        Self {
            screen_width,
            screen_height,
            hud_scale,
            ratio,
            ratio_normalized,
        }
    }

    pub fn rescale_width_from_fhd(&self, value: u32) -> u32 {
        self.rescale(
            self.apply_ratio_on_width(value * self.screen_width / 1920),
            self.screen_width / 2,
        )
    }
    pub fn rescale_height_from_fhd(&self, value: u32) -> u32 {
        self.rescale(
            self.apply_ratio_on_height(value * self.screen_height / 1080),
            self.screen_height,
        )
    }

    pub fn rescale(&self, value: u32, scale_point: u32) -> u32 {
        (scale_point as f32 - self.hud_scale * (scale_point as f32 - value as f32)) as u32
    }

    fn apply_ratio_on_width(&self, value: u32) -> u32 {
        let mid = (self.screen_width / 2) as i32;
        let mut from_mid = (mid - value as i32) as f64;
        if self.ratio < 1.3 {
            from_mid *= 0.85;
        }
        (mid - (from_mid / self.ratio_normalized) as i32) as u32
    }
    fn apply_ratio_on_height(&self, value: u32) -> u32 {
        if self.ratio < 1.3 {
            let from_bottom = self.screen_height - value;
            return self.screen_height - (from_bottom * 845 / 1000);
        }
        value
    }
}
