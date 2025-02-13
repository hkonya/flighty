use crate::{config::{Config, ConfigFile}, Result};
use std::{path::Path, process::Command};
use tracing::{debug, info, warn};

pub async fn execute(path: &str) -> Result<()> {
    let path = Path::new(path);
    info!("Proje başlatılıyor: {}", path.display());

    // Flutter projesini kontrol et
    validate_flutter_project(path)?;

    // .flighty dizinini oluştur
    let flighty_dir = path.join(".flighty");
    if !flighty_dir.exists() {
        std::fs::create_dir(&flighty_dir)?;
        info!(".flighty dizini oluşturuldu");
    }

    // Yapılandırma dosyasını oluştur
    let config_file = ConfigFile::from_path(flighty_dir.join("config.yaml"));
    if config_file.exists() {
        info!("Yapılandırma dosyası zaten mevcut");
        return Ok(());
    }

    // Varsayılan yapılandırmayı oluştur
    let _config = config_file.create_default::<Config>()?;
    info!("Varsayılan yapılandırma oluşturuldu");

    Ok(())
}

fn validate_flutter_project(path: &Path) -> Result<()> {
    // pubspec.yaml kontrolü
    let pubspec_path = path.join("pubspec.yaml");
    if !pubspec_path.exists() {
        return Err(crate::Error::InvalidArgument(
            "Flutter projesi bulunamadı. Lütfen önce 'flutter create' komutunu çalıştırın.".into(),
        ));
    }

    // Android dizini kontrolü
    let android_dir = path.join("android");
    if !android_dir.exists() {
        warn!("Android dizini bulunamadı");
    }

    // iOS dizini kontrolü
    let ios_dir = path.join("ios");
    if !ios_dir.exists() {
        warn!("iOS dizini bulunamadı");
    }

    // Flutter doctor kontrolü
    debug!("Flutter doctor çalıştırılıyor...");
    let output = Command::new("flutter")
        .arg("doctor")
        .output()
        .map_err(|_| crate::Error::FlutterSdkNotFound)?;

    if !output.status.success() {
        return Err(crate::Error::FlutterCli(
            "Flutter doctor başarısız oldu. Lütfen Flutter kurulumunuzu kontrol edin.".into(),
        ));
    }

    info!("Flutter projesi doğrulandı");
    Ok(())
} 