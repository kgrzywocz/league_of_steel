mod events;
mod steelseries_engine;

use steelseries_engine::EventSender;

pub struct SteelConnector {
    server_connector: steelseries_engine::ServerConnector,
}

impl SteelConnector {
    pub fn new() -> Result<Self, SteelLibError> {
        Ok(Self {
            server_connector: steelseries_engine::create_server_connector()?,
        })
    }

    pub fn register_game(&self) {
        log::info!("Game register sent");
        self.server_connector.send(events::RegisterGame::new());
    }
    pub fn register_game_events(&self) {
        log::info!("Game events register sent");
        self.server_connector
            .send(events::RegisterHealthEvent::new());
        self.server_connector.send(events::RegisterManaEvent::new());
        self.server_connector.send(events::RegisterHitEvent::new());
    }

    pub fn send_health(&self, health: u8) {
        log::debug!("Health={} send", health);
        self.server_connector.send(events::Health::new(health));
    }
    pub fn send_mana(&self, mana: u8) {
        log::debug!("Mana={} send", mana);
        self.server_connector.send(events::Mana::new(mana));
    }
    pub fn send_hit(&self, hit: u8) {
        log::debug!("Hit={} send", hit);
        self.server_connector.send(events::Hit::new(hit));
    }
}

#[derive(Debug)]
pub enum SteelLibError {
    SSEConfig(String),
}
impl std::fmt::Display for SteelLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SteelLibError::SSEConfig(problem) => write!(f, "{}!", problem),
        }
    }
}
impl std::error::Error for SteelLibError {}
