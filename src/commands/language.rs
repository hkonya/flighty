use crate::{config::{Config, ConfigFile}, i18n, Result};
use std::collections::HashMap;
use tracing::info;

pub async fn execute(code: String) -> Result<()> {
    // Dili değiştir
    if i18n::set_language(&code) {
        // Yapılandırma dosyasını güncelle
        let config_file = ConfigFile::new()?;
        let mut config: Config = config_file.load()?;
        
        // Yapılandırmayı güncelle
        config.language = code.clone();
        config_file.save(&config)?;

        let mut args = HashMap::new();
        args.insert("lang", code);
        
        info!("{}", i18n::translate_with_args("msg-language-changed", Some(&args)));
        Ok(())
    } else {
        Err(crate::Error::InvalidArgument(format!(
            "Desteklenmeyen dil kodu: {}",
            code
        )))
    }
} 