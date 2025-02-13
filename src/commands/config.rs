use crate::{config::{Config, ConfigFile}, Result};
use tracing::info;

pub fn execute(key: Option<String>, value: Option<String>) -> Result<()> {
    let config_file = ConfigFile::new()?;
    let config: Config = config_file.load()?;

    match (key.as_deref(), value) {
        (Some("language"), None) => {
            println!("Mevcut dil: {}", config.language);
            Ok(())
        }
        (Some(key), None) => {
            tracing::info!("Yapılandırma değeri gösterme henüz uygulanmadı");
            Ok(())
        }
        (Some(key), Some(value)) => {
            tracing::info!("Yapılandırma değeri ayarlama henüz uygulanmadı");
            Ok(())
        }
        (None, _) => {
            println!("Mevcut yapılandırma:");
            println!("  Dil: {}", config.language);
            Ok(())
        }
    }
} 