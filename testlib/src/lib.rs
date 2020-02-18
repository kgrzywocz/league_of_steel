pub mod image;
pub mod resolution;
pub mod server;
mod process;
pub use process::Process;
pub use process::start_lol;
mod lol_config;
pub use lol_config::LolConfig;
