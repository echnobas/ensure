#[derive(Debug)]
pub enum EnsureError {
    FetchErr(ureq::Error),
    FeedErr(rss::Error),
    IoError(std::io::Error),
    Malformed,
}

impl std::fmt::Display for EnsureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EnsureError {}

impl From<rss::Error> for EnsureError {
    fn from(e: rss::Error) -> Self {
        Self::FeedErr(e)
    }
}

impl From<ureq::Error> for EnsureError {
    fn from(e: ureq::Error) -> Self {
        Self::FetchErr(e)
    }
}

impl From<std::io::Error> for EnsureError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
