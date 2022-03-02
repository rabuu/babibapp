#[derive(Debug)]
pub struct BabibappApiError {
    pub error: String,
}

impl std::fmt::Display for BabibappApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Babibapp API Error: {}", self.error)
    }
}

impl std::error::Error for BabibappApiError {}

impl From<reqwest::Error> for BabibappApiError {
    fn from(e: reqwest::Error) -> Self {
        BabibappApiError {
            error: format!("{}", e),
        }
    }
}
