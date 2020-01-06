#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error {0}")]
    Network(#[from] reqwest::Error),
    #[error("json error {0}")]
    JSON(#[from] serde_json::error::Error),
    #[error("yaml error {0}")]
    YAML(#[from] serde_yaml::Error),
    #[error("io error {0}")]
    IO(#[from] std::io::Error),
}