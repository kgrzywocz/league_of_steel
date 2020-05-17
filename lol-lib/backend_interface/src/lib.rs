mod bindings;
mod backend;
pub use backend::*;
pub use bindings::{
    LolStats,
    BackendCaptureRect,
    BackendScreenResolution,
    BackendColor,
    BackendScreenAnalyzer,
    BackendPixelRect,
};
