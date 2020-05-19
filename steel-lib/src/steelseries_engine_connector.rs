use crate::sse_events;
use crate::sse_server_connector;

use crate::SSEEventSender;
use crate::SteelLibError;
use game_lib::game_events::{GameInfo, MultipleGameEvents};

pub struct SteelSeriesEngineConnector {
    server_connector: Box<dyn SSEEventSender>,
}

impl SteelSeriesEngineConnector {
    pub fn new() -> Result<Self, SteelLibError> {
        Ok(Self {
            server_connector: Box::new(sse_server_connector::create_server_connector()?),
        })
    }

    pub fn register_games(&self, game_infos: &Vec<GameInfo>) -> Result<(), SteelLibError> {
        for game in game_infos {
            self.register(game)?;
        }
        Ok(())
    }

    pub fn register(&self, game_info: &GameInfo) -> Result<(), SteelLibError> {
        log::info!("Game register sent");
        self.server_connector
            .send(&sse_events::RegisterGame::from(game_info))?;
        log::info!("Game events register sent");

        for event in &game_info.game_events {
            self.server_connector
                .send(&sse_events::RegisterGameEvent::from(
                    &game_info.game,
                    &event,
                ))?;
        }
        Ok(())
    }

    pub fn send_events(&self, events: MultipleGameEvents) -> Result<(), SteelLibError> {
        Self::log_events(&events);
        self.server_connector.send(&events)
    }

    fn log_events(events: &MultipleGameEvents) {
        let mut msg = format!("MultipleGameEvents for = {} :", events.game);
        for event in &events.events {
            msg.push_str(&format!(" {}={}", event.name, event.value));
        }
        log::info!("{}", &msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SSEEvent;
    use game_lib::game_events::*;
    use mockall::*;

    #[test]
    fn test_game_registration() {
        let game_info = get_lol_game_info();

        let mut mock_server = MockServerConnector::new();
        expect_lol_registration_events(&mut mock_server);

        let sut = SteelSeriesEngineConnector {
            server_connector: Box::new(mock_server),
        };
        sut.register(&game_info).expect("register_game failed");
    }

    fn get_lol_game_info() -> GameInfo {
        GameInfo::new(
            "LEAGUE_OF_STEEL",
            "League of Legends",
            "Riot Games",
            vec![
                GameEventInfo::new("HEALTH").set_type(GameEventType::Health),
                GameEventInfo::new("MANA").set_type(GameEventType::Mana),
                GameEventInfo::new("HIT").set_type(GameEventType::Boom),
            ],
        )
    }

    fn expect_lol_registration_events(mock_server: &mut MockServerConnector) {
        expect_event(
            mock_server,
            String::from("game_metadata"),
            String::from(
                r#"{
            "game": "LEAGUE_OF_STEEL",
            "game_display_name": "League of Legends",
            "developer": "Riot Games / SSE3 plugin Krzysztof Grzywocz"
            }"#,
            ),
        );

        expect_event(
            mock_server,
            String::from("register_game_event"),
            String::from(
                r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "HEALTH",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 1,
            "value_optional": false
          }"#,
            ),
        );

        expect_event(
            mock_server,
            String::from("register_game_event"),
            String::from(
                r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "MANA",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 14,
            "value_optional": false
          }"#,
            ),
        );

        expect_event(
            mock_server,
            String::from("register_game_event"),
            String::from(
                r#"{
            "game": "LEAGUE_OF_STEEL",
            "event": "HIT",
            "min_value": 0,
            "max_value": 100,
            "icon_id": 5,
            "value_optional": false
          }"#,
            ),
        );
    }

    #[test]
    fn test_lol_events() {
        let mut mock_server = MockServerConnector::new();
        expect_event(
            &mut mock_server,
            String::from("multiple_game_events"),
            String::from(
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
                },
 {
                     "event": "HIT",
                "data": {
                    "value": 4
                }
                }
            ]
        }"#,
            ),
        );

        let sut = SteelSeriesEngineConnector {
            server_connector: Box::new(mock_server),
        };
        sut.send_events(MultipleGameEvents::new(
            "LEAGUE_OF_STEEL",
            vec![
                GameEvent::new("HEALTH", 80),
                GameEvent::new("MANA", 50),
                GameEvent::new("HIT", 4),
            ],
        ))
        .expect("register_game failed");
    }

    mock! {
        ServerConnector{}
        trait SSEEventSender{
            fn send<'a>(&self, event: &'a (dyn SSEEvent + 'a)) -> Result<(), SteelLibError>;
        }
    }
    fn expect_event(mock_server: &mut MockServerConnector, endpoint: String, body: String) {
        mock_server
            .expect_send()
            .withf(move |x| {
                if x.endpoint() != endpoint {
                    println!(
                        "endpoint missmatch\nin call:\n{}\nexpected:\n{}",
                        x.endpoint(),
                        endpoint
                    );
                    return false;
                }
                if x.body() != body {
                    println!(
                        "body missmatch\nin call:\n{}\nexpected:\n{}",
                        x.body(),
                        body
                    );
                    println!("{} vs {}", x.body().len(), body.len());
                    return false;
                }
                return true;
            })
            .returning(|_| Ok(()));
    }
}
