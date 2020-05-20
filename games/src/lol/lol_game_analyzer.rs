use super::bars_position::BarsPosition;
use super::lollib::GAME_NAME;
use backend_interface::*;
use backend_win::*;
use game_lib::game_events::*;
use game_lib::GameAnalyzer;

pub struct LolStats {
    pub health: u8,
    pub mana: u8,
    pub hit: u8,
}

pub struct LolGameAnalyzer {
    lol_pixel_rect_analyzer: LolPixelRectAnalyzer,
    backend: Backend,
    bars_position: BarsPosition,
}

impl GameAnalyzer for LolGameAnalyzer {
    fn pool_events(&mut self) -> MultipleGameEvents {
        self.adjust_hud_position();
        self.backend
            .analyze_screenshot(&mut self.lol_pixel_rect_analyzer);
        let stats = &self.lol_pixel_rect_analyzer.lol_stats;

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
            lol_pixel_rect_analyzer: LolPixelRectAnalyzer::new(),
            backend: Backend::new(),
            bars_position: BarsPosition::new(),
        }
    }

    fn adjust_hud_position(&mut self) {
        let hud_scale = self.get_hud_global_scale_from_config().unwrap_or(1.0);
        self.bars_position.set_hud_scaling(hud_scale);

        let capture_rect = self.bars_position.get(&self.backend.get_mode());
        self.backend.set_capture_rect(&capture_rect);

        self.log_analizie_params(hud_scale, &capture_rect);
    }
    fn log_analizie_params(&self, hud_scale: f32, capture_rect: &BackendCaptureRect) {
        let mode = self.backend.get_mode();
        log::info!(
            "Screen {}x{} hud_scale={} capture={}x{} {}x{}",
            mode.width,
            mode.height,
            hud_scale,
            capture_rect.left,
            capture_rect.top,
            capture_rect.right,
            capture_rect.bottom
        );
    }

    fn get_hud_global_scale_from_config(&self) -> Option<f32> {
        let lol_path = get_process_exe_path("League of Legends.exe");
        log::debug!("lol_path={}", lol_path);
        crate::lol::lolconfig::get_hud_global_scale(&lol_path)
    }
}

struct LolPixelRectAnalyzer {
    lol_stats: LolStats,
}

impl PixelRectAnalyzer for LolPixelRectAnalyzer {
    fn analyze_function(&mut self, rect: &PixelRect) {
        self.lol_stats = Self::get_stats(rect)
    }
}

impl LolPixelRectAnalyzer {
    fn new() -> Self {
        Self {
            lol_stats: LolStats {
                health: 0,
                mana: 0,
                hit: 0,
            },
        }
    }
    fn get_stats(rect: &PixelRect) -> LolStats {
        let health = Self::analyze_bar(rect, 0, |c| c.is_green(), 0);
        LolStats {
            health: health,
            mana: Self::analyze_bar(rect, rect.get_hight(), |c| c.is_blue() || c.is_yellow(), 0),
            hit: Self::analyze_bar(rect, 0, |c| c.is_red(), health),
        }
    }

    fn analyze_bar<F>(rect: &PixelRect, row: i32, predicate: F, start_from_percent: u8) -> u8
    where
        F: Fn(&Color) -> bool,
    {
        let max_gap = 20;

        let row_len = rect.get_width();
        let offset = start_from_percent as i32 * row_len / 100;

        let mut last = offset;
        for j in offset..row_len {
            if predicate(&rect.get_color(row, j)) {
                last = j;
            } else if j > last + max_gap {
                break;
            }
        }

        return (100 * (last + 1) / row_len) as u8 - start_from_percent;
    }
}
