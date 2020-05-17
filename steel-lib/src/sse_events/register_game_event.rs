use crate::SSEEvent;
use game_lib::game_events::*;

pub struct RegisterGameEvent {
    body: String,
}
impl RegisterGameEvent {
    pub fn from(game: &str, event_info: &GameEventInfo) -> Self {
        let body = format!(
            r#"{{
            "game": "{}",
            "event": "{}",
            "min_value": {},
            "max_value": {},
            "icon_id": {},
            "value_optional": false
          }}"#,
            game,
            event_info.name,
            event_info.min_value,
            event_info.max_value,
            Self::get_type_id(&event_info.event_type)
        );
        Self { body }
    }
    fn get_type_id(event_type: &GameEventType) -> u8 {
        //icon api: https://github.com/SteelSeries/gamesense-sdk/blob/0ec272cbbdc57f1fdba4ddc0173079ad35366591/doc/api/event-icons.md
        match event_type {
            GameEventType::Default => 0,
            GameEventType::Health => 1,
            GameEventType::Armor => 2,
            GameEventType::Ammo => 3,
            GameEventType::Boom => 5,
            GameEventType::Mana => 14,
        }
    }
}
impl SSEEvent for RegisterGameEvent {
    fn endpoint(&self) -> String {
        String::from("register_game_event")
    }
    fn body(&self) -> String {
        self.body.clone()
    }
}
