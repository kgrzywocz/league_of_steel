pub mod image;
mod process;
pub mod resolution;
pub mod server;
pub use process::start_lol;
pub use process::Process;
mod lol_config;
pub use lol_config::LolConfig;
