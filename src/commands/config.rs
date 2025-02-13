use crate::{fl, config::{Config, ConfigFile}, Result};

pub fn execute(key: Option<String>, value: Option<String>) -> Result<()> {
    let config_file = ConfigFile::new()?;
    let config: Config = config_file.load()?;

    match (key.as_deref(), value) {
        (Some("language"), None) => {
            println!("{}", fl!("config-language", lang = config.language));
            Ok(())
        }
        (Some(_key), None) => {
            tracing::info!("{}", fl!("config-show-not-implemented"));
            Ok(())
        }
        (Some(_key), Some(_value)) => {
            tracing::info!("{}", fl!("config-set-not-implemented"));
            Ok(())
        }
        (None, _) => {
            println!("{}", fl!("config-current"));
            println!("  {}", fl!("config-language", lang = config.language));
            Ok(())
        }
    }
} 