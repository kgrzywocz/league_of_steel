use super::bars_position::BarsPosition;
use super::lollib::{GAME_EXE, GAME_NAME};

use super::lol_pixel_analyzer::LolPixelAnalyzer;
use backend_interface::*;
use game_lib::game_events::*;
use game_lib::GameAnalyzer;

pub struct LolGameAnalyzer {
    backend: Backend,
    pixel_analyzer: LolPixelAnalyzer,
}

impl GameAnalyzer for LolGameAnalyzer {
    fn pool_events(&mut self) -> MultipleGameEvents {
        self.adjust_hud_position();
        self.backend.analyze_screenshot(&mut self.pixel_analyzer);

        let stats = self.pixel_analyzer.get_stats();
        MultipleGameEvents::new(
            GAME_NAME,
            vec![
                GameEvent::new("HEALTH", stats.health),
                GameEvent::new("MANA", stats.mana),
                GameEvent::new("HIT", stats.hit),
            ],
        )
    }
}

impl LolGameAnalyzer {
    pub fn new() -> Self {
        Self {
            backend: Backend::new(),
            pixel_analyzer: LolPixelAnalyzer::new(),
        }
    }

    fn adjust_hud_position(&mut self) {
        let hud_scale = self.get_hud_global_scale_from_config().unwrap_or(1.0);
        let mode = self.backend.get_mode();

        let bars_position = BarsPosition::new(mode.width, mode.height, hud_scale);
        self.pixel_analyzer.update_bars_positions(&bars_position);

        Self::log_analyze_params(&mode, hud_scale, &bars_position);
    }
    fn log_analyze_params(
        mode: &BackendScreenResolution,
        hud_scale: f32,
        bars_position: &BarsPosition,
    ) {
        log::info!(
            "Screen {}x{} hud_scale={} pos(range={}-{} hp={} mana={})",
            mode.width,
            mode.height,
            hud_scale,
            bars_position.get_range().0,
            bars_position.get_range().1,
            bars_position.get_health_height(),
            bars_position.get_mana_height(),
        );
    }

    fn get_hud_global_scale_from_config(&self) -> Option<f32> {
        let lol_path = get_process_exe_path(GAME_EXE);
        log::debug!("lol_path={}", lol_path);
        crate::lol::lolconfig::get_hud_global_scale(&lol_path)
    }
}
