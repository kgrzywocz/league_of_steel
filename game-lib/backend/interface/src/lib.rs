mod backend;
mod bindings;
mod process;
pub use backend::*;
pub use bindings::{
    BackendColor, BackendPixelRect, BackendScreenAnalyzer, BackendScreenResolution,
};
pub use process::*;
