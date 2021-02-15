use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
  #[error("toml deserializing failed: '{0}'")]
  TomlDeserializeError(#[from] toml::de::Error),
  #[error("toml serializing failed: '{0}'")]
  TomlSerializeError(#[from] toml::ser::Error),

  #[error("io error: '{0}'")]
  IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ConfigError>;