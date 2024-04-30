#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Discord rate limit exceeded, try again later.")]
    RateLimit,
    #[error("Not found.")]
    NotFound,
    #[error("Failed to lookup host.")]
    WhoisHostLookup,
    #[error("Failed to lookup IP.")]
    WhoisIpLookup,
    #[error("Failed to serialize or deserialize.")]
    Serde,
    #[error("Failed to send request.")]
    Request,
    #[error("Unknown error.")]
    Unknown,
}
