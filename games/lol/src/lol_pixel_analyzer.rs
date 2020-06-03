use super::bars_position::BarsPosition;
use super::spells::{Spells, SpellsValues};
use game_lib::screen_elements::Bar;
use game_lib::*;

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
    spells_values: SpellsValues,
    spells: Spells,
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

        self.spells_values = self.spells.get_spells_values(pixels);
    }
}

impl LolPixelAnalyzer {
    pub fn new() -> Self {
        Self {
            lol_stats: LolStats::new(),
            spells_values: SpellsValues::new(),
            spells: Spells::new(),
            health_bar: Bar::new(),
            mana_bar: Bar::new(),
        }
    }
    pub fn get_stats(&self) -> &LolStats {
        &self.lol_stats
    }
    pub fn get_spells(&self) -> &SpellsValues {
        &self.spells_values
    }

    pub fn update_hud_positions(&mut self, width: u32, height: u32, hud_global_scale: f32) {
        let bars_position = BarsPosition::new(width, height, hud_global_scale);
        self.update_bars_positions(&bars_position);
        self.spells
            .update_positions(width, height, hud_global_scale);

        Self::log_analyze_params(width, height, hud_global_scale, &bars_position);
    }

    fn update_bars_positions(&mut self, bars_position: &BarsPosition) {
        self.health_bar
            .set_position(bars_position.get_health_height());
        self.health_bar.set_range(bars_position.get_range());

        self.mana_bar.set_position(bars_position.get_mana_height());
        self.mana_bar.set_range(bars_position.get_range());
    }

    fn log_analyze_params(width: u32, height: u32, hud_scale: f32, bars_position: &BarsPosition) {
        log::info!(
            "Screen {}x{} hud_scale={} pos(range={}-{} hp={} mana={})",
            width,
            height,
            hud_scale,
            bars_position.get_range().0,
            bars_position.get_range().1,
            bars_position.get_health_height(),
            bars_position.get_mana_height(),
        );
    }
}
