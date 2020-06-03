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
        let spells = self.pixel_analyzer.get_spells();
        MultipleGameEvents::new(
            GAME_NAME,
            vec![
                GameEvent::new("HEALTH", stats.health),
                GameEvent::new("MANA", stats.mana),
                GameEvent::new("HIT", stats.hit),
                GameEvent::new("SPELL_Q", spells.q),
                GameEvent::new("SPELL_W", spells.w),
                GameEvent::new("SPELL_E", spells.e),
                GameEvent::new("SPELL_R", spells.r),
                GameEvent::new("SPELL_D", spells.d),
                GameEvent::new("SPELL_F", spells.f),
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

        self.pixel_analyzer
            .update_hud_positions(mode.width, mode.height, hud_scale);
    }

    fn get_hud_global_scale_from_config(&self) -> Option<f32> {
        let lol_path = get_process_exe_path(GAME_EXE);
        log::debug!("lol_path={}", lol_path);
        crate::lol::lolconfig::get_hud_global_scale(&lol_path)
    }
}
