use crate::steelseries_engine::Event;

pub struct Health {
    health: u8,
}
impl Health {
    pub fn new(health: u8) -> Self {
        Self { health }
    }
}
impl Event for Health {
    fn endpoint(&self) -> String {
        String::from("game_event")
    }
    fn body(&self) -> String {
        format!(
            r#"{{
            "game": "LEAGUE_OF_STEEL",
            {}
            }}"#,
            event_str("HEALTH", self.health)
        )
    }
}

pub struct Mana {
    mana: u8,
}
impl Mana {
    pub fn new(mana: u8) -> Self {
        Self { mana }
    }
}
impl Event for Mana {
    fn endpoint(&self) -> String {
        String::from("game_event")
    }
    fn body(&self) -> String {
        format!(
            r#"{{
            "game": "LEAGUE_OF_STEEL",
            {}
            }}"#,
            event_str("MANA", self.mana)
        )
    }
}

pub struct Hit {
    hit: u8,
}
impl Hit {
    pub fn new(hit: u8) -> Self {
        Self { hit }
    }
}
impl Event for Hit {
    fn endpoint(&self) -> String {
        String::from("game_event")
    }
    fn body(&self) -> String {
        format!(
            r#"{{
            "game": "LEAGUE_OF_STEEL",
            {}
            }}"#,
            event_str("HIT", self.hit)
        )
    }
}

pub struct MultipleStats {
    health: u8,
    mana: u8,
    hit: u8,
}
impl MultipleStats {
    pub fn new(health: u8, mana: u8, hit: u8) -> Self {
        Self { health, mana, hit }
    }
}
impl Event for MultipleStats {
    fn endpoint(&self) -> String {
        String::from("multiple_game_events")
    }
    fn body(&self) -> String {
        let res = format!(
            r#"{{
            "game": "LEAGUE_OF_STEEL",
            "events": [
            {{
                 {}
            }},
            {{
                 {}
            }},
            {{
                 {}
            }}
            ]
        }}"#,
            event_str("HEALTH", self.health),
            event_str("MANA", self.mana),
            event_str("HIT", self.hit),
        );
        res
    }
}

fn event_str(name: &str, value: u8) -> String {
    format!(
        r#""event": "{}",
                "data": {{
                    "value": {}
                }}"#,
        name,
        value.to_string()
    )
}

pub struct RegisterGame {}
impl RegisterGame {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for RegisterGame {
    fn endpoint(&self) -> String {
        String::from("game_metadata")
    }
    fn body(&self) -> String {
        String::from(
            r#"{
            "game": "LEAGUE_OF_STEEL",
            "game_display_name": "League of Legends",
            "developer": "Riot Games / SSE3 plugin Krzysztof Grzywocz"
            }"#,
        )
    }
}

//icon api: https://github.com/SteelSeries/gamesense-sdk/blob/0ec272cbbdc57f1fdba4ddc0173079ad35366591/doc/api/event-icons.md

pub struct RegisterHealthEvent {}
impl RegisterHealthEvent {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for RegisterHealthEvent {
    fn endpoint(&self) -> String {
        String::from("register_game_event")
    }
    fn body(&self) -> String {
        String::from(
            r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "HEALTH",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 1,
            "value_optional": false
          }"#,
        )
    }
}

pub struct RegisterManaEvent {}
impl RegisterManaEvent {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for RegisterManaEvent {
    fn endpoint(&self) -> String {
        String::from("register_game_event")
    }
    fn body(&self) -> String {
        String::from(
            r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "MANA",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 14,
            "value_optional": false
          }"#,
        )
    }
}

pub struct RegisterHitEvent {}
impl RegisterHitEvent {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for RegisterHitEvent {
    fn endpoint(&self) -> String {
        String::from("register_game_event")
    }
    fn body(&self) -> String {
        String::from(
            r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "HIT",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 5,
            "value_optional": false
          }"#,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_health_event_string() {
        let event = r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "HEALTH",
            "data": {
                "value": 75
            }
        }"#;
        assert_eq!(event, Health::new(75).body());
    }
}
