#[derive(Debug)]
pub enum SteelLibError {
    SSEConfig(String),
    SentError(String),
}
impl std::fmt::Display for SteelLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SteelLibError::SSEConfig(problem) => write!(f, "Problem with SSE3: {}!", problem),
            SteelLibError::SentError(problem) => write!(f, "Sent error: {}!", problem),
        }
    }
}
impl std::error::Error for SteelLibError {}
