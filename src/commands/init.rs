use crate::{config::{Config, ConfigFile}, Result};
use std::path::Path;
use tracing::info;

pub async fn execute(path: &str) -> Result<()> {
    let path = Path::new(path);
    info!("Proje başlatılıyor: {}", path.display());

    // Yapılandırma dosyasını oluştur
    let config_file = ConfigFile::from_path(path.join(".flighty.yaml"));
    if config_file.exists() {
        info!("Yapılandırma dosyası zaten mevcut");
        return Ok(());
    }

    // Varsayılan yapılandırmayı oluştur
    let config = config_file.create_default::<Config>()?;
    info!("Varsayılan yapılandırma oluşturuldu");

    // Flutter projesini kontrol et
    if !path.join("pubspec.yaml").exists() {
        info!("Flutter projesi bulunamadı. Lütfen önce 'flutter create' komutunu çalıştırın.");
    }

    Ok(())
} 