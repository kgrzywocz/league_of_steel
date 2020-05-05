use steel_lib;
use steel_lib::SteelLibError;

pub fn wait_for_steel_connector(
    sse_seek_interval: std::time::Duration,
) -> steel_lib::SteelConnector {
    let mut steel_connector;
    loop {
        let res = steel_lib::SteelConnector::new();
        match res {
            Ok(res) => {
                steel_connector = res;
                if let Err(e) = register_game(&steel_connector) {
                    log::warn!("{}", e);
                } else {
                    break;
                };
            }
            Err(e) => {
                log::warn!("{}", e);
            }
        }
        std::thread::sleep(sse_seek_interval);
    }
    steel_connector
}

fn register_game(steel_connector: &steel_lib::SteelConnector) -> Result<(), SteelLibError> {
    steel_connector.register_game()?;
    steel_connector.register_game_events()
}
