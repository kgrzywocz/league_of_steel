use super::bars_position::BarsPosition;
use crate::screen_elements::Bar;
use backend_interface::*;

pub struct LolStats {
    pub health: u8,
    pub mana: u8,
    pub hit: u8,
}
impl LolStats {
    fn new() -> Self {
        Self {
            health: 0,
            mana: 0,
            hit: 0,
        }
    }
}

pub struct LolPixelAnalyzer {
    lol_stats: LolStats,
    health_bar: Bar,
    mana_bar: Bar,
}

impl PixelRectAnalyzer for LolPixelAnalyzer {
    fn analyze_function(&mut self, pixels: &PixelRect) {
        let health = self.health_bar.analyze_pixels(pixels, |c| c.is_green(), 0);
        let mana = self
            .mana_bar
            .analyze_pixels(pixels, |c| c.is_blue() || c.is_yellow(), 0);
        let hit = self
            .health_bar
            .analyze_pixels(pixels, |c| c.is_red(), health as u32);

        self.lol_stats = LolStats { health, mana, hit };
    }
}

impl LolPixelAnalyzer {
    pub fn new() -> Self {
        Self {
            lol_stats: LolStats::new(),
            health_bar: Bar::new(),
            mana_bar: Bar::new(),
        }
    }
    pub fn get_stats(&self) -> &LolStats {
        &self.lol_stats
    }

    pub fn update_bars_positions(&mut self, bars_position: &BarsPosition) {
        self.health_bar
            .set_position(bars_position.get_health_height());
        self.health_bar.set_range(bars_position.get_range());

        self.mana_bar.set_position(bars_position.get_mana_height());
        self.mana_bar.set_range(bars_position.get_range());
    }
}
