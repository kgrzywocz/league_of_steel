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

    pub fn register_game(&self) -> Result<(), SteelLibError> {
        log::info!("Game register sent");
        self.server_connector.send(events::RegisterGame::new())
    }
    pub fn register_game_events(&self) -> Result<(), SteelLibError> {
        log::info!("Game events register sent");
        self.server_connector
            .send(events::RegisterHealthEvent::new())?;
        self.server_connector
            .send(events::RegisterManaEvent::new())?;
        self.server_connector.send(events::RegisterHitEvent::new())
    }

    pub fn send_health(&self, health: u8) -> Result<(), SteelLibError> {
        log::info!("Health={} send", health);
        self.server_connector.send(events::Health::new(health))
    }
    pub fn send_mana(&self, mana: u8) -> Result<(), SteelLibError> {
        log::info!("Mana={} send", mana);
        self.server_connector.send(events::Mana::new(mana))
    }
    pub fn send_hit(&self, hit: u8) -> Result<(), SteelLibError> {
        log::info!("Hit={} send", hit);
        self.server_connector.send(events::Hit::new(hit))
    }
    pub fn send_stats(&self, health: u8, mana: u8, hit: u8) -> Result<(), SteelLibError> {
        log::info!("Health={} Mana={} Hit={} send", health, mana, hit);
        self.server_connector
            .send(events::MultipleStats::new(health, mana, hit))
    }
}

#[derive(Debug)]
pub enum SteelLibError {
    SSEConfig(String),
    SentError(String),
}
impl std::fmt::Display for SteelLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SteelLibError::SSEConfig(problem) => write!(f, "Problem with SSE3: {}!", problem),
            SteelLibError::SentError(problem) => write!(f, "Sent error: {}!", problem),
        }
    }
}
impl std::error::Error for SteelLibError {}
