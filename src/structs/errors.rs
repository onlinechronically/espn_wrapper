use thiserror::Error;

#[derive(Debug, Error)]
pub enum EspnError {
    #[error("req")]
    Request(#[from] ureq::Error),
    #[error("json")]
    JSON(#[from] std::io::Error),
}
