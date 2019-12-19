extern crate reqwest;

mod config;
mod server_connector;

pub use server_connector::ServerConnector;
pub use super::SteelLibError;

pub trait Event {
    fn endpoint(&self) -> String;
    fn body(&self) -> String;
}

pub trait EventSender {
    fn send<E>(&self, event: E)
    where
        E: Event;
}

pub fn create_server_connector() -> Result<ServerConnector, SteelLibError> {
    let config = config::Config::new()?;
    let address = config.get_server_address()?;
    Ok(ServerConnector::new(&address))
}
