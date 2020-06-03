use super::hud_rescale::Rescaler;
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
    rescaler: Rescaler,
}

impl Spells {
    pub fn new() -> Self {
        Self {
            screen_width: 0,
            screen_height: 0,
            hud_scale: 1.0,
            rescaler: Rescaler::new(0, 0, 1.0),
        }
    }
    pub fn update_positions(&mut self, width: u32, height: u32, hud_scale: f32) {
        self.screen_width = width;
        self.screen_height = height;
        self.hud_scale = hud_scale;
        self.rescaler = Rescaler::new(width, height, hud_scale);
    }
    pub fn get_spells_values(&self, pixels: &PixelRect) -> SpellsValues {
        SpellsValues {
            q: self.spell_value(pixels, self.get_spell_q_width()),
            w: self.spell_value(pixels, self.get_spell_w_width()),
            e: self.spell_value(pixels, self.get_spell_e_width()),
            r: self.spell_value(pixels, self.get_spell_r_width()),
        }
    }
    fn spell_value(&self, pixels: &PixelRect, x: u32) -> u8 {
        pixels.get_color(x, self.get_spells_height()).is_yellow() as u8
    }

    fn get_spells_height(&self) -> u32 {
        self.rescaler.rescale_height_from_fhd(948)
    }
    fn get_spell_q_width(&self) -> u32 {
        self.rescaler.rescale_width_from_fhd(760)
    }
    fn get_spell_w_width(&self) -> u32 {
        self.rescaler.rescale_width_from_fhd(825)
    }
    fn get_spell_e_width(&self) -> u32 {
        self.rescaler.rescale_width_from_fhd(891)
    }
    fn get_spell_r_width(&self) -> u32 {
        self.rescaler.rescale_width_from_fhd(958)
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
        assert_eq!(sut.get_spell_q_width(), 760);
        assert_eq!(sut.get_spell_w_width(), 825);
        assert_eq!(sut.get_spell_e_width(), 891);
        assert_eq!(sut.get_spell_r_width(), 958);
    }
    #[test]
    fn test_1280x720_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1280, 720, 1.0);

        assert_eq!(sut.get_spells_height(), 632);
        assert_eq!(sut.get_spell_q_width(), 506);
        assert_eq!(sut.get_spell_w_width(), 550);
        assert_eq!(sut.get_spell_e_width(), 594);
        assert_eq!(sut.get_spell_r_width(), 638);
    }
    #[test]
    fn test_1920x1080_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1920, 1080, 0.01);

        assert_eq!(sut.get_spells_height(), 993);
        assert_eq!(sut.get_spell_q_width(), 829);
        assert_eq!(sut.get_spell_w_width(), 871);
        assert_eq!(sut.get_spell_e_width(), 914);
        assert_eq!(sut.get_spell_r_width(), 958);
    }
    #[test]
    fn test_1920x1080_hud_50() {
        let mut sut = Spells::new();
        sut.update_positions(1920, 1080, 0.5);

        assert_eq!(sut.get_spells_height(), 971);
        assert_eq!(sut.get_spell_q_width(), 795);
        assert_eq!(sut.get_spell_w_width(), 848);
        assert_eq!(sut.get_spell_e_width(), 903);
        assert_eq!(sut.get_spell_r_width(), 958);
    }

    #[test]
    fn test_1280x1024_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1280, 1024, 1.0);

        assert_eq!(sut.get_spells_height(), 918);
        assert_eq!(sut.get_spell_q_width(), 479);
        assert_eq!(sut.get_spell_w_width(), 532);
        assert_eq!(sut.get_spell_e_width(), 585);
        assert_eq!(sut.get_spell_r_width(), 638);
    }
    #[test]
    fn test_800x600_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(800, 600, 1.0);

        assert_eq!(sut.get_spells_height(), 526);
        assert_eq!(sut.get_spell_q_width(), 288);
        assert_eq!(sut.get_spell_w_width(), 324);
        assert_eq!(sut.get_spell_e_width(), 362);
        assert_eq!(sut.get_spell_r_width(), 399);
    }
    #[test]
    fn test_1024x768_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1024, 768, 1.0);

        assert_eq!(sut.get_spells_height(), 674);
        assert_eq!(sut.get_spell_q_width(), 370);
        assert_eq!(sut.get_spell_w_width(), 416);
        assert_eq!(sut.get_spell_e_width(), 463);
        assert_eq!(sut.get_spell_r_width(), 510);
    }
    #[test]
    fn test_1152x864_hud_1() {
        let mut sut = Spells::new();
        sut.update_positions(1152, 864, 1.0);

        assert_eq!(sut.get_spells_height(), 758);
        assert_eq!(sut.get_spell_q_width(), 416);
        assert_eq!(sut.get_spell_w_width(), 468);
        assert_eq!(sut.get_spell_e_width(), 520);
        assert_eq!(sut.get_spell_r_width(), 574);
    }
    #[test]
    fn test_1600x1024_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1600, 1024, 0.01);

        assert_eq!(sut.get_spells_height(), 941);
        assert_eq!(sut.get_spell_q_width(), 675);
        assert_eq!(sut.get_spell_w_width(), 716);
        assert_eq!(sut.get_spell_e_width(), 757);
        assert_eq!(sut.get_spell_r_width(), 798);
    }
    #[test]
    fn test_1680x1050_hud_001() {
        let mut sut = Spells::new();
        sut.update_positions(1680, 1050, 0.01);

        assert_eq!(sut.get_spells_height(), 965);
        assert_eq!(sut.get_spell_q_width(), 713);
        assert_eq!(sut.get_spell_w_width(), 753);
        assert_eq!(sut.get_spell_e_width(), 796);
        assert_eq!(sut.get_spell_r_width(), 838);
    }
}
