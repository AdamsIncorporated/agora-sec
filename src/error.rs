use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum EDGARParserError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] Box<dyn std::error::Error>),

    #[error("Failed to parse JSON: {0}")]
    JSONParseError(#[from] serde_json::Error),

    #[error("Requested resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Received invalid response: {0}")]
    InvalidResponse(String),

    #[error("URL Parsing error.")]
    UrlParseError(ParseError),

    #[error("EDGAR Filing not found.")]
    FilingTypeNotFound(),

    #[error("Invalid date format supplied {0}, it should be yyyymmdd.")]
    InvalidDateFormat(String),

    #[error("EDGAR Owner Type not found.")]
    OwnerTypeNotFound(),
}

impl From<ParseError> for EDGARParserError {
    fn from(err: ParseError) -> Self {
        EDGARParserError::UrlParseError(err)
    }
}
