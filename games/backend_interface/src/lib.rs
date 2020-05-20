mod backend;
mod bindings;
pub use backend::*;
pub use bindings::{
    BackendCaptureRect, BackendColor, BackendPixelRect, BackendScreenAnalyzer,
    BackendScreenResolution,
};
