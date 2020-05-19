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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_game() {
        let sut = RegisterGame::from(&GameInfo::new(
            "LEAGUE_OF_STEEL",
            "League of Legends",
            "Riot Games",
            vec![],
        ));
        assert_eq!(sut.endpoint(), "game_metadata");
        assert_eq!(
            sut.body(),
            String::from(
                r#"{
            "game": "LEAGUE_OF_STEEL",
            "game_display_name": "League of Legends",
            "developer": "Riot Games / SSE3 plugin Krzysztof Grzywocz"
            }"#
            )
        );
    }
}
