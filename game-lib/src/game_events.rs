pub struct MultipleGameEvents {
    pub game: String,
    pub events: Vec<GameEvent>,
}
impl MultipleGameEvents {
    pub fn new(game: &str, events: Vec<GameEvent>) -> Self {
        Self {
            game: game.to_string(),
            events: events,
        }
    }
}

pub struct GameEvent {
    pub name: String,
    pub value: u8,
}
impl GameEvent {
    pub fn new(name: &str, value: u8) -> Self {
        Self {
            name: name.to_string(),
            value: value,
        }
    }
}

pub struct GameInfo {
    pub game: String,
    pub game_display_name: String,
    pub developer: String,
    pub game_events: Vec<GameEventInfo>,
}
impl GameInfo {
    pub fn new(
        game: &str,
        game_display_name: &str,
        developer: &str,
        game_events: Vec<GameEventInfo>,
    ) -> Self {
        Self {
            game: game.to_string(),
            game_display_name: game_display_name.to_string(),
            developer: developer.to_string(),
            game_events: game_events,
        }
    }
}

pub struct GameEventInfo {
    pub name: String,
    pub min_value: i32,
    pub max_value: i32,
    pub event_type: GameEventType,
}
impl GameEventInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            min_value: 0,
            max_value: 100,
            event_type: GameEventType::Default,
        }
    }
    pub fn set_type(mut self, event_type: GameEventType) -> Self {
        self.event_type = event_type;
        self
    }
}

pub enum GameEventType {
    Default,
    Health,
    Armor,
    Ammo,
    Boom,
    Mana,
}
