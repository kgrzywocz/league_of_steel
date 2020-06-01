use crate::screen_elements::*;
use backend_interface::*;
use game_lib::game_events::*;
use game_lib::{GameAnalyzer, GameTrait};

pub const GAME_NAME: &str = "FORTNITE";

pub struct FortniteLib {}

impl FortniteLib {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameTrait for FortniteLib {
    fn get_game_info(&self) -> GameInfo {
        GameInfo::new(
            GAME_NAME,
            "Fortnite",
            "Epic Games",
            vec![
                GameEventInfo::new("HEALTH").set_type(GameEventType::Health),
                GameEventInfo::new("ARMOR").set_type(GameEventType::Armor),
            ],
        )
    }
    fn is_running(&self) -> bool {
        is_process_running("FortniteClient-Win64-Shipping_BE.exe")
            || is_process_running("FortniteClient-Win64-Shipping_EAC.exe")
    }

    fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer> {
        Box::new(FortniteGameAnalyzer::new())
    }
}

pub struct FortniteGameAnalyzer {
    backend: Backend,
    pixel_analyzer: FortnitePixelAnalyzer,
}

impl GameAnalyzer for FortniteGameAnalyzer {
    fn pool_events(&mut self) -> MultipleGameEvents {
        self.backend.analyze_screenshot(&mut self.pixel_analyzer);

        MultipleGameEvents::new(
            GAME_NAME,
            vec![
                GameEvent::new("HEALTH", self.pixel_analyzer.get_health()),
                GameEvent::new("ARMOR", self.pixel_analyzer.get_armor()),
            ],
        )
    }
}

impl FortniteGameAnalyzer {
    pub fn new() -> Self {
        Self {
            backend: Backend::new(),
            pixel_analyzer: FortnitePixelAnalyzer::new(),
        }
    }
}

pub struct FortnitePixelAnalyzer {
    health: u8,
    armor: u8,
}

impl PixelRectAnalyzer for FortnitePixelAnalyzer {
    fn analyze_function(&mut self, pixels: &PixelRect) {
        let positions = BarsPosition::new(pixels.get_width(), pixels.get_hight());
        let range = positions.get_range();

        let mut health_bar = Bar::new();
        health_bar.set_position(positions.get_health_hight());
        health_bar.set_range(range);
        let mut armor_bar = Bar::new();
        armor_bar.set_position(positions.get_armor_hight());
        armor_bar.set_range(range);

        self.health = health_bar.analyze_pixels(pixels, |c| c.is_green(), 0);
        self.armor = armor_bar.analyze_pixels(pixels, |c| c.is_blue(), 0);
    }
}

impl FortnitePixelAnalyzer {
    pub fn new() -> Self {
        Self {
            health: 0,
            armor: 0,
        }
    }
    pub fn get_health(&self) -> u8 {
        self.health
    }
    pub fn get_armor(&self) -> u8 {
        self.armor
    }
}

struct BarsPosition {
    width: u32,
    hight: u32,
}
impl BarsPosition {
    fn new(width: u32, hight: u32) -> Self {
        Self { width, hight }
    }
    fn get_range(&self) -> (u32, u32) {
        (self.width * 100 / 1920, self.width * 510 / 1920)
    }
    fn get_health_hight(&self) -> u32 {
        let ratio = (self.width as f64 / self.hight as f64) / (1920.0 / 1080.0);
        let from_bottom = (self.hight - (self.hight * 965 / 1080)) as f64;
        self.hight - (from_bottom * ratio) as u32
    }
    fn get_armor_hight(&self) -> u32 {
        let ratio = (self.width as f64 / self.hight as f64) / (1920.0 / 1080.0);
        let from_bottom = (self.hight - (self.hight * 960 / 1080)) as f64;
        self.hight - (from_bottom * ratio) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bars_position_1920x1080() {
        let sut = BarsPosition::new(1920, 1080);

        assert_eq!(sut.get_range().0, 100);
        assert_eq!(sut.get_range().1, 510);
        assert_eq!(sut.get_health_hight(), 965);
        assert_eq!(sut.get_armor_hight(), 960);
    }
    #[test]
    fn test_bars_position_1280x720() {
        let sut = BarsPosition::new(1280, 720);

        assert_eq!(sut.get_range().0, 66);
        assert_eq!(sut.get_range().1, 340);
        assert_eq!(sut.get_health_hight(), 643);
        assert_eq!(sut.get_armor_hight(), 640);
    }
    #[test]
    fn test_bars_position_1366x768() {
        let sut = BarsPosition::new(1366, 768);

        assert_eq!(sut.get_range().0, 71);
        assert_eq!(sut.get_range().1, 362);
        assert_eq!(sut.get_health_hight(), 686);
        assert_eq!(sut.get_armor_hight(), 682);
    }
    #[test]
    fn test_bars_position_1280x1024() {
        let sut = BarsPosition::new(1280, 1024);

        assert_eq!(sut.get_range().0, 66);
        assert_eq!(sut.get_range().1, 340);
        assert_eq!(sut.get_health_hight(), 947);
        assert_eq!(sut.get_armor_hight(), 944);
    }
    #[test]
    fn test_bars_position_1600x900() {
        let sut = BarsPosition::new(1600, 900);

        assert_eq!(sut.get_range().0, 83);
        assert_eq!(sut.get_range().1, 425);
        assert_eq!(sut.get_health_hight(), 804);
        assert_eq!(sut.get_armor_hight(), 800);
    }
    #[test]
    fn test_bars_position_1600x1024() {
        let sut = BarsPosition::new(1600, 1024);

        assert_eq!(sut.get_range().0, 83);
        assert_eq!(sut.get_range().1, 425);
        assert_eq!(sut.get_health_hight(), 928);
        assert_eq!(sut.get_armor_hight(), 924);
    }
    #[test]
    fn test_bars_position_1680x1050() {
        let sut = BarsPosition::new(1680, 1050);

        assert_eq!(sut.get_range().0, 87);
        assert_eq!(sut.get_range().1, 446);
        assert_eq!(sut.get_health_hight(), 950);
        assert_eq!(sut.get_armor_hight(), 945);
    }
}
