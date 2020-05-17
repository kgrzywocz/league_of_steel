use steel_lib;

pub fn wait_for_steel_connector(
    sse_seek_interval: std::time::Duration,
) -> steel_lib::SteelConnector {
    let mut steel_connector;
    loop {
        let res = steel_lib::SteelConnector::new();
        match res {
            Ok(res) => {
                steel_connector = res;
                if let Err(e) = steel_connector.register_game() {
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
