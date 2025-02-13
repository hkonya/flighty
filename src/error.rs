use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Yapılandırma hatası: {0}")]
    Config(String),

    #[error("Flutter CLI hatası: {0}")]
    FlutterCli(String),

    #[error("IO hatası: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serileştirme hatası: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML hatası: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Geçersiz semver: {0}")]
    Semver(#[from] semver::Error),

    #[error("HTTP isteği hatası: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Geçersiz argüman: {0}")]
    InvalidArgument(String),

    #[error("Flutter SDK bulunamadı")]
    FlutterSdkNotFound,

    #[error("Proje yapılandırması bulunamadı")]
    ProjectConfigNotFound,

    #[error("Bilinmeyen hata: {0}")]
    Unknown(String),
} 