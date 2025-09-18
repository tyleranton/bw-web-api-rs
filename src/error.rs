use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    /// Error while creating the HTTP client
    #[error("Failed to create HTTP client: {0}")]
    ClientCreationError(String),

    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Failed to deserialize the API response
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),

    /// API returned an error response
    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    /// Invalid input parameters
    #[error("Invalid input: {0}")]
    ValidationError(String),
}

// Implement standard error conversion
impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::DeserializationError(err.to_string())
    }
}
