use super::lol_game_analyzer;
use backend_win::*;
use game_lib::game_events::*;
use game_lib::{GameAnalyzer, GameTrait};

pub const GAME_NAME: &str = "LEAGUE_OF_STEEL";

pub struct LolLib {}

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
        Box::new(lol_game_analyzer::LolGameAnalyzer::new())
    }
}
