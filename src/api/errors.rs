#[derive(Debug)]
pub enum APIErrors {
    AuthenticationFailed,
    InvalidRequest,
    ResourceNotFound,
    InternalServerError,
}
impl std::error::Error for APIErrors {}

impl std::fmt::Display for APIErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APIErrors::InternalServerError => write!(f, "Internal server error"),
            APIErrors::AuthenticationFailed => write!(f, "Authentication failed"),
            APIErrors::InvalidRequest => write!(f, "Invalid request"),
            APIErrors::ResourceNotFound => write!(f, "Resource not found"),
        }
    }
}
