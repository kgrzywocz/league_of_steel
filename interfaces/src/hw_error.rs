#[derive(Debug)]
pub enum HwError {
    ConfigError(String),
    SentError(String),
}
impl std::fmt::Display for HwError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HwError::ConfigError(problem) => write!(f, "Problem with HW: {}!", problem),
            HwError::SentError(problem) => write!(f, "Sent error: {}!", problem),
        }
    }
}
impl std::error::Error for HwError {}
