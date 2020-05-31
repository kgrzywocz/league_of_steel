pub use steelseries_engine_connector::SteelSeriesEngineConnector as SteelConnector;

mod sse_events;
mod sse_server_connector;
mod steelseries_engine_connector;

trait SSEEvent {
    fn endpoint(&self) -> String;
    fn body(&self) -> String;
}

trait SSEEventSender {
    fn send(&self, event: &dyn SSEEvent) -> Result<(), game_lib::HwError>;
}
