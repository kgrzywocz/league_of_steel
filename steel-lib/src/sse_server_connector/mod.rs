pub use server_connector::ServerConnector;

mod config;
mod server_connector;

pub fn create_server_connector() -> Result<ServerConnector, interfaces::HwError> {
    let config = config::Config::new()?;
    let address = config.get_server_address()?;
    Ok(ServerConnector::new(&address))
}
