use crate::SSEEvent;
use game_lib::game_events::*;

pub struct RegisterGame {
    body: String,
}
impl RegisterGame {
    pub fn from(info: &GameInfo) -> Self {
        Self {
            body: format!(
                r#"{{
            "game": "{}",
            "game_display_name": "{}",
            "developer": "{} / SSE3 plugin Krzysztof Grzywocz"
            }}"#,
                info.game, info.game_display_name, info.developer
            ),
        }
    }
}
impl SSEEvent for RegisterGame {
    fn endpoint(&self) -> String {
        String::from("game_metadata")
    }
    fn body(&self) -> String {
        self.body.clone()
    }
}
