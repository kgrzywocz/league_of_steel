use super::hud_rescale::rescale;
use backend_interface::PixelRect;

pub struct SpellsValues {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}
impl SpellsValues {
    pub fn new() -> Self {
        Self {
            q: 0,
            w: 0,
            e: 0,
            r: 0,
        }
    }
}

pub struct Spells {
    screen_width: u32,
    screen_height: u32,
    hud_scale: f32,
}

impl Spells {
    pub fn new() -> Self {
        Self {
            screen_width: 0,
            screen_height: 0,
            hud_scale: 1.0,
        }
    }
    pub fn update_positions(&mut self, width: u32, height: u32, hud_scale: f32) {
        self.screen_width = width;
        self.screen_height = height;
        self.hud_scale = hud_scale;
    }
    pub fn get_spells_values(&self, pixels: &PixelRect) -> SpellsValues {
        SpellsValues {
            q: self.spell_value(pixels, self.get_q_spells_width()),
            w: self.spell_value(pixels, self.get_w_spells_width()),
            e: self.spell_value(pixels, self.get_e_spells_width()),
            r: self.spell_value(pixels, self.get_r_spells_width()),
        }
    }
    fn spell_value(&self, pixels: &PixelRect, x: u32) -> u8 {
        pixels.get_color(x, self.get_spells_height()).is_yellow() as u8
    }

    fn get_spells_height(&self) -> u32 {
        let value = self.apply_ratio_on_height(self.screen_height * 948 / 1080);

        rescale(self.hud_scale, value, self.screen_height)
    }
    fn get_q_spells_width(&self) -> u32 {
        rescale(
            self.hud_scale,
            self.apply_ratio_on_width(self.screen_width * 760 / 1920),
            self.screen_width / 2,
        )
    }
    fn get_w_spells_width(&self) -> u32 {
        rescale(
            self.hud_scale,
            self.apply_ratio_on_width(self.screen_width * 825 / 1920),
            self.screen_width / 2,
        )
    }
    fn get_e_spells_width(&self) -> u32 {
        rescale(
            self.hud_scale,
            self.apply_ratio_on_width(self.screen_width * 891 / 1920),
            self.screen_width / 2,
        )
    }
    fn get_r_spells_width(&self) -> u32 {
        rescale(
            self.hud_scale,
            self.apply_ratio_on_width(self.screen_width * 958 / 1920),
            self.screen_width / 2,
        )
    }
    fn apply_ratio_on_height(&self, value: u32) -> u32 {
        let ratio = self.screen_width as f64 / self.screen_height as f64;
        if ratio < 1.3 {
            return value * 918 / 898;
        }
        value
    }
    fn apply_ratio_on_width(&self, value: u32) -> u32 {
        let ratio = self.screen_width as f64 / self.screen_height as f64;
        let ratio_normalized = ratio / (1920.0 / 1080.0);
        let mid = self.screen_width / 2;
        let mut from_mid = (mid - value) as f64;
        if ratio < 1.3 {
            from_mid *= 0.85;
        }
        let value = mid - (from_mid / ratio_normalized) as u32;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1920x1080_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1920, 1080, 1.0);

        assert_eq!(sut.get_spells_height(), 948);
        assert_eq!(sut.get_q_spells_width(), 760);
        assert_eq!(sut.get_w_spells_width(), 825);
        assert_eq!(sut.get_e_spells_width(), 891);
        assert_eq!(sut.get_r_spells_width(), 958);
    }
    #[test]
    fn test_1280x720_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1280, 720, 1.0);

        assert_eq!(sut.get_spells_height(), 632);
        assert_eq!(sut.get_q_spells_width(), 506);
        assert_eq!(sut.get_w_spells_width(), 550);
        assert_eq!(sut.get_e_spells_width(), 594);
        assert_eq!(sut.get_r_spells_width(), 638);
    }
    #[test]
    fn test_1920x1080_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1920, 1080, 0.01);

        assert_eq!(sut.get_spells_height(), 993);
        assert_eq!(sut.get_q_spells_width(), 829);
        assert_eq!(sut.get_w_spells_width(), 871);
        assert_eq!(sut.get_e_spells_width(), 914);
        assert_eq!(sut.get_r_spells_width(), 958);
    }
    #[test]
    fn test_1920x1080_hud_50() {
        let mut sut = Spells::new();
        sut.update_positions(1920, 1080, 0.5);

        assert_eq!(sut.get_spells_height(), 971);
        assert_eq!(sut.get_q_spells_width(), 795);
        assert_eq!(sut.get_w_spells_width(), 848);
        assert_eq!(sut.get_e_spells_width(), 903);
        assert_eq!(sut.get_r_spells_width(), 958);
    }

    #[test]
    fn test_1280x1024_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1280, 1024, 1.0);

        assert_eq!(sut.get_spells_height(), 918);
        assert_eq!(sut.get_q_spells_width(), 479);
        assert_eq!(sut.get_w_spells_width(), 532);
        assert_eq!(sut.get_e_spells_width(), 585);
        assert_eq!(sut.get_r_spells_width(), 638);
    }
    #[test]
    fn test_800x600_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(800, 600, 1.0);

        assert_eq!(sut.get_spells_height(), 526);
        assert_eq!(sut.get_q_spells_width(), 288);
        assert_eq!(sut.get_w_spells_width(), 324);
        assert_eq!(sut.get_e_spells_width(), 362);
        assert_eq!(sut.get_r_spells_width(), 399);
    }
    #[test]
    fn test_1024x768_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1024, 768, 1.0);

        assert_eq!(sut.get_spells_height(), 674);
        assert_eq!(sut.get_q_spells_width(), 370);
        assert_eq!(sut.get_w_spells_width(), 416);
        assert_eq!(sut.get_e_spells_width(), 463);
        assert_eq!(sut.get_r_spells_width(), 510);
    }
    #[test]
    fn test_1152x864_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1152, 864, 1.0);

        assert_eq!(sut.get_spells_height(), 758);
        assert_eq!(sut.get_q_spells_width(), 416);
        assert_eq!(sut.get_w_spells_width(), 468);
        assert_eq!(sut.get_e_spells_width(), 520);
        assert_eq!(sut.get_r_spells_width(), 574);
    }
    #[test]
    fn test_1600x1024_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1600, 1024, 0.01);

        assert_eq!(sut.get_spells_height(), 941);
        assert_eq!(sut.get_q_spells_width(), 675);
        assert_eq!(sut.get_w_spells_width(), 716);
        assert_eq!(sut.get_e_spells_width(), 757);
        assert_eq!(sut.get_r_spells_width(), 798);
    }
    #[test]
    fn test_1680x1050_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1680, 1050, 0.01);

        assert_eq!(sut.get_spells_height(), 965);
        assert_eq!(sut.get_q_spells_width(), 713);
        assert_eq!(sut.get_w_spells_width(), 753);
        assert_eq!(sut.get_e_spells_width(), 796);
        assert_eq!(sut.get_r_spells_width(), 838);
    }
}
