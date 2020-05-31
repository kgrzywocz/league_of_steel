use crate::game_events::*;
use crate::HwError;

pub trait HwConnector {
    fn register_games(&self, game_infos: &Vec<GameInfo>) -> Result<(), HwError>;
    fn register(&self, game_info: &GameInfo) -> Result<(), HwError>;
    fn send_events(&self, events: MultipleGameEvents) -> Result<(), HwError>;
}
