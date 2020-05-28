use std::time::Duration;

const SSE_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const GAME_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const UPDATE_INTERVAL: Duration = Duration::from_millis(150);

pub struct LeagueOfSteelConfig {
    pub sse_seek_interval: Duration,
    pub game_seek_interval: Duration,
    pub update_interval: Duration,
}
impl LeagueOfSteelConfig {
    pub fn new() -> Self {
        Self {
            sse_seek_interval: SSE_SEEK_INTERVAL,
            game_seek_interval: GAME_SEEK_INTERVAL,
            update_interval: UPDATE_INTERVAL,
        }
    }
}
