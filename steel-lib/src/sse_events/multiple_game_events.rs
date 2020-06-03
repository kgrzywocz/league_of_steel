use crate::SSEEvent;
use interfaces::game_events::*;

impl SSEEvent for MultipleGameEvents {
    fn endpoint(&self) -> String {
        String::from("multiple_game_events")
    }
    fn body(&self) -> String {
        let mut events_str = String::new();
        for event in &self.events {
            events_str.push_str(&format!(
                "{{
                     {}
                }},\n ",
                event_str(&event.name, event.value)
            ));
        }
        events_str.pop();
        events_str.pop();
        events_str.pop();
        return format!(
            r#"{{
            "game": "{}",
            "events": [
                {}
            ]
        }}"#,
            self.game, events_str
        );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_game_events() {
        let sut = MultipleGameEvents::new(
            "LEAGUE_OF_STEEL",
            vec![GameEvent::new("HEALTH", 80), GameEvent::new("MANA", 50)],
        );

        assert_eq!(sut.endpoint(), "multiple_game_events");
        assert_eq!(
            sut.body(),
            r#"{
            "game": "LEAGUE_OF_STEEL",
            "events": [
                {
                     "event": "HEALTH",
                "data": {
                    "value": 80
                }
                },
 {
                     "event": "MANA",
                "data": {
                    "value": 50
                }
                }
            ]
        }"#
        );
    }
}
