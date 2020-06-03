use crate::game_events::*;

pub trait GameTrait {
    fn get_game_info(&self) -> GameInfo;
    fn is_running(&self) -> bool;
    fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer>;
}

pub trait GameAnalyzer {
    fn pool_events(&mut self) -> MultipleGameEvents;
}
