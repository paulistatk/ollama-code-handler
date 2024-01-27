use std::fmt;
use reqwest::StatusCode;


#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP error: {}", self.status)
    }
}

impl std::error::Error for HttpError {}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn test_http_error_display() {
        let error = HttpError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
        };
        assert_eq!(
            format!("{}", error),
            "HTTP error: 500 Internal Server Error"
        );
    }
}

