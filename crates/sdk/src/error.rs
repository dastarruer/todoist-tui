use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unable to retrieve API key")]
    KeyNotProvided,
    #[error("too many requests made: {0}")]
    RateLimited(String),
    #[error("invalid authentication: {0}")]
    AuthenticationError(String),
    #[error("unauthorized request: {0}")]
    AuthorizationError(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalidated request: {0}")]
    ValidationError(String),
    #[error("server failure (status `{status}`): {message}")]
    ServerError { status: u16, message: String },
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("error while parsing json: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("unexpected empty response from `{endpoint}`: {message}")]
    EmptyResponse { endpoint: String, message: String },
    #[error("error while parsing date string: {0}")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("error while parsing timezone string: {0}")]
    InvalidTimezone(#[from] chrono_tz::ParseError),
    #[error("error: {0}")]
    Generic(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        let Some(status) = value.status() else {
            // responses with no status are theoretically impossible, so they
            // can only happen if there's a network error
            return Self::NetworkError(value.to_string());
        };
        match status {
            StatusCode::UNAUTHORIZED => Self::AuthenticationError(value.to_string()),
            StatusCode::FORBIDDEN => Self::AuthorizationError(value.to_string()),
            StatusCode::NOT_FOUND => Self::NotFound(value.to_string()),
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimited(value.to_string()),
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
                Self::ValidationError(value.to_string())
            }
            status if status.is_server_error() => Self::ServerError {
                status: status.as_u16(),
                message: value.to_string(),
            },
            _ => Self::Generic(value.to_string()),
        }
    }
}
