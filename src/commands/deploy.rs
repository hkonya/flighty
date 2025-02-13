use crate::{config::{Config, ConfigFile}, Result};
use std::process::Command;
use tracing::{debug, info};

pub async fn execute(platform: Option<String>, target: Option<String>) -> Result<()> {
    let config_file = ConfigFile::new()?;
    let config: Config = config_file.load()?;

    // Platform kontrolü
    let platforms = match platform.as_deref() {
        Some("android") => vec!["android"],
        Some("ios") => vec!["ios"],
        Some(_) => return Err(crate::Error::InvalidArgument("Geçersiz platform".into())),
        None => vec!["android", "ios"],
    };

    // Hedef kontrolü
    let target = target.as_deref().unwrap_or("store");
    if !["store", "firebase"].contains(&target) {
        return Err(crate::Error::InvalidArgument("Geçersiz hedef".into()));
    }

    for platform in platforms {
        info!("Dağıtım başlatılıyor: {} -> {}", platform, target);

        match (platform, target) {
            ("android", "store") => deploy_android_store(&config)?,
            ("ios", "store") => deploy_ios_store(&config)?,
            (_, "firebase") => deploy_firebase(&config, platform)?,
            _ => unreachable!(),
        }

        info!("{} dağıtımı tamamlandı", platform);
    }

    Ok(())
}

fn deploy_android_store(_config: &Config) -> Result<()> {
    // TODO: Implement Google Play deployment
    info!("Google Play dağıtımı henüz uygulanmadı");
    Ok(())
}

fn deploy_ios_store(_config: &Config) -> Result<()> {
    // TODO: Implement App Store deployment
    info!("App Store dağıtımı henüz uygulanmadı");
    Ok(())
}

fn deploy_firebase(config: &Config, platform: &str) -> Result<()> {
    let artifact_path = match platform {
        "android" => "build/app/outputs/flutter-apk/app-release.apk",
        "ios" => "build/ios/ipa/Runner.ipa",
        _ => unreachable!(),
    };

    let mut cmd = Command::new("firebase");
    cmd.args([
        "appdistribution:distribute",
        artifact_path,
        "--app",
        match platform {
            "android" => "ANDROID_APP_ID", // TODO: Config'den al
            "ios" => "IOS_APP_ID",        // TODO: Config'den al
            _ => unreachable!(),
        },
    ]);

    debug!("Çalıştırılacak komut: {:?}", cmd);
    let status = cmd.status()?;

    if !status.success() {
        return Err(crate::Error::FlutterCli(format!(
            "Firebase dağıtımı başarısız oldu: {}",
            platform
        )));
    }

    Ok(())
} 