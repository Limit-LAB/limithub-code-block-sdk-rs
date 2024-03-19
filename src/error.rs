pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty {0} ports")]
    EmptyPorts(&'static str),

    #[error("Name conflict: {0}")]
    NameConflict(String),

    #[error("Missing port: {0}")]
    MissingPort(String),
}
