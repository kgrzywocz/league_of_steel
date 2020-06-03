use super::lol_game_analyzer;
use game_lib::is_process_running;
use interfaces::game_events::*;
use interfaces::{GameAnalyzer, GameTrait};

pub const GAME_NAME: &str = "LEAGUE_OF_STEEL";
pub const GAME_EXE: &str = "League of Legends.exe";

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
                GameEventInfo::new("SPELL_Q").set_max_value(1),
                GameEventInfo::new("SPELL_W").set_max_value(1),
                GameEventInfo::new("SPELL_E").set_max_value(1),
                GameEventInfo::new("SPELL_R").set_max_value(1),
                GameEventInfo::new("SPELL_D").set_max_value(1),
                GameEventInfo::new("SPELL_F").set_max_value(1),
            ],
        )
    }
    fn is_running(&self) -> bool {
        is_process_running(GAME_EXE)
    }

    fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer> {
        Box::new(lol_game_analyzer::LolGameAnalyzer::new())
    }
}
