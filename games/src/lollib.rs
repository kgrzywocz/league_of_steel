use game_lib::game_events::*;

use backend_interface::*;
use backend_win::*;
use game_lib::{GameAnalyzer, GameTrait};

const GAME_NAME: &str = "LEAGUE_OF_STEEL";

pub struct LolLib {}
pub struct LolGameAnalyzer {
    backend: Backend,
    bars_position: BarsPosition,
}

impl LolLib {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameTrait for LolLib {
    fn get_game_info(&self) -> GameInfo {
        GameInfo::new(
            GAME_NAME,
            "League of Legends",
            "Riot Games",
            vec![
                GameEventInfo::new("HEALTH").set_type(GameEventType::Health),
                GameEventInfo::new("MANA").set_type(GameEventType::Mana),
                GameEventInfo::new("HIT").set_type(GameEventType::Boom),
            ],
        )
    }
    fn is_running(&self) -> bool {
        is_process_running("League of Legends.exe")
    }

    fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer> {
        Box::new(LolGameAnalyzer {
            backend: Backend::new(LolGameAnalyzer::analyze_function),
            bars_position: BarsPosition::new(),
        })
    }
}
impl GameAnalyzer for LolGameAnalyzer {
    fn pool_events(&mut self) -> MultipleGameEvents {
        self.adjust_hud_scale();
        let stats = self.backend.analyze_screenshot();

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
    fn analyze_function(rect: PixelRect) -> LolStats {
        PixelRectAnalyzer::get_stats(&rect)
    }

    fn adjust_hud_scale(&mut self) {
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
        crate::lolconfig::get_hud_global_scale(&lol_path)
    }
}

struct BarsPosition {
    hud_scale: f32,
}
impl BarsPosition {
    fn new() -> Self {
        Self { hud_scale: 1.0 }
    }
    fn get(&self, disp_mode: &BackendScreenResolution) -> BackendCaptureRect {
        let mut pos = BackendCaptureRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        //682x1031 1094x1065 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66
        //381x744 575x761 on 1024x768 - hud 0%(0.01) -> 0.66
        //245x572 474x591 on 800x600
        //297x581 449x594 on 800x600 hud 0
        //314x732 607x757 on 1024x768
        //354x824 684x851 on 1152x864

        let mut ratio = (disp_mode.width as f32 / disp_mode.height as f32) / 1.77777777777;
        ratio = (ratio + 2.0) / 3.0;

        pos.left = (ratio * 0.356 * (disp_mode.width as f32)) as i32;
        pos.right = (ratio * 0.57 * (disp_mode.width as f32)) as i32; //bar len
        pos.right = ((disp_mode.width as f32)
            - (ratio * (disp_mode.width as i32 - pos.right) as f32)) as i32; //space on rigth
        pos.bottom = ((1.0 - 0.015 * ratio) * (disp_mode.height as f32)) as i32 - 1;
        pos.top = pos.bottom - (ratio * ratio * 0.025 * (disp_mode.height as f32)) as i32 + 1;

        self.rescale_for_hud_scaling(&mut pos, disp_mode);

        return pos;
    }
    fn set_hud_scaling(&mut self, hud_global_scale: f32) {
        self.hud_scale = 0.333 * hud_global_scale + 0.666;
    }

    fn rescale_for_hud_scaling(
        &self,
        pos: &mut BackendCaptureRect,
        disp_mode: &BackendScreenResolution,
    ) {
        let mid_width = disp_mode.width / 2;
        pos.left =
            (mid_width as f32 - self.hud_scale * (mid_width as f32 - pos.left as f32)) as i32;
        pos.right =
            (mid_width as f32 + self.hud_scale * (pos.right as f32 - mid_width as f32)) as i32;
        pos.top = (disp_mode.height as f32
            - self.hud_scale * (disp_mode.height as f32 - pos.top as f32)) as i32;
        pos.bottom = (disp_mode.height as f32
            - self.hud_scale * (disp_mode.height as f32 - pos.bottom as f32))
            as i32;
    }
}

struct PixelRectAnalyzer;

impl PixelRectAnalyzer {
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
