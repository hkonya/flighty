use crate::{config::{Config, ConfigFile}, Result};
use std::{fs, path::{Path, PathBuf}, process::Command};
use tracing::{debug, info, warn};

const FLIGHTY_DIRS: &[&str] = &[
    "config/environments",
    "secrets/android/keystore",
    "secrets/android/google-play",
    "secrets/ios/certificates",
    "secrets/ios/profiles",
    "logs/builds",
    "logs/deployments",
    "logs/errors",
    "cache/builds",
    "cache/dependencies",
];

const DEFAULT_CONFIG_FILES: &[(&str, &str)] = &[
    ("config/config.yaml", include_str!("../../templates/config.yaml")),
    ("config/defaults.yaml", include_str!("../../templates/defaults.yaml")),
    ("config/hooks.yaml", include_str!("../../templates/hooks.yaml")),
    ("config/environments/development.yaml", include_str!("../../templates/environments/development.yaml")),
    ("config/environments/staging.yaml", include_str!("../../templates/environments/staging.yaml")),
    ("config/environments/production.yaml", include_str!("../../templates/environments/production.yaml")),
];

pub async fn execute(path: &str) -> Result<()> {
    let path = Path::new(path);
    info!("Proje başlatılıyor: {}", path.display());

    // Flutter projesini kontrol et
    validate_flutter_project(path)?;

    // .flighty dizin yapısını oluştur
    let flighty_dir = path.join(".flighty");
    create_flighty_structure(&flighty_dir)?;

    // .gitignore dosyasını güncelle
    update_gitignore(path)?;

    info!("Proje başarıyla başlatıldı");
    Ok(())
}

fn create_flighty_structure(root: &Path) -> Result<()> {
    info!("Flighty dizin yapısı oluşturuluyor...");

    // Ana dizini oluştur
    if !root.exists() {
        fs::create_dir(root)?;
        debug!("Ana dizin oluşturuldu: {}", root.display());
    }

    // Alt dizinleri oluştur
    for dir in FLIGHTY_DIRS {
        let dir_path = root.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
            debug!("Dizin oluşturuldu: {}", dir_path.display());
        }
    }

    // Varsayılan yapılandırma dosyalarını oluştur
    for (file, template) in DEFAULT_CONFIG_FILES {
        let file_path = root.join(file);
        if !file_path.exists() {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&file_path, template)?;
            debug!("Yapılandırma dosyası oluşturuldu: {}", file_path.display());
        }
    }

    info!("Flighty dizin yapısı oluşturuldu");
    Ok(())
}

fn update_gitignore(project_root: &Path) -> Result<()> {
    let gitignore_path = project_root.join(".gitignore");
    let mut content = String::new();

    // Mevcut .gitignore dosyasını oku
    if gitignore_path.exists() {
        content = fs::read_to_string(&gitignore_path)?;
    }

    // Flighty özel girdilerini ekle
    let flighty_entries = r#"
# Flighty özel dosyaları
.flighty/secrets/
.flighty/logs/
.flighty/cache/
"#;

    if !content.contains(".flighty/secrets/") {
        content.push_str(flighty_entries);
        fs::write(gitignore_path, content)?;
        info!(".gitignore dosyası güncellendi");
    }

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