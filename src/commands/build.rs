use crate::{config::{Config, ConfigFile, ReleaseChannel}, Result};
use std::process::Command;
use tracing::{debug, info};

pub async fn execute(
    platform: Option<String>,
    version: Option<String>,
    build_number: Option<u32>,
    channel: Option<String>,
) -> Result<()> {
    let config_file = ConfigFile::new()?;
    let config: Config = config_file.load()?;

    // Platform kontrolü
    let platforms = match platform.as_deref() {
        Some("android") => vec!["android"],
        Some("ios") => vec!["ios"],
        Some(_) => return Err(crate::Error::InvalidArgument("Geçersiz platform".into())),
        None => vec!["android", "ios"],
    };

    // Release kanalını belirle
    let release_channel = if let Some(channel) = channel {
        match channel.to_lowercase().as_str() {
            "alpha" => ReleaseChannel::Alpha,
            "beta" => ReleaseChannel::Beta,
            "production" => ReleaseChannel::Production,
            _ => return Err(crate::Error::InvalidArgument("Geçersiz kanal".into())),
        }
    } else {
        config.build.release_channel
    };

    // Her platform için build işlemini başlat
    for platform in platforms {
        info!("Build başlatılıyor: {}", platform);

        let mut cmd = Command::new("flutter");
        cmd.arg("build").arg(platform);

        // Release modu
        cmd.arg("--release");

        // Versiyon ve build numarası
        if let Some(ref version) = version {
            cmd.arg("--build-name").arg(version);
        }
        if let Some(build_number) = build_number {
            cmd.arg("--build-number").arg(build_number.to_string());
        }

        // Platform özel ayarlar
        match platform {
            "android" => {
                if let Some(ref keystore) = config.android.keystore_path {
                    cmd.arg("--keystore").arg(keystore);
                }
            }
            "ios" => {
                if let Some(ref bundle_id) = config.ios.bundle_id {
                    cmd.arg("--bundle-identifier").arg(bundle_id);
                }
            }
            _ => unreachable!(),
        }

        debug!("Çalıştırılacak komut: {:?}", cmd);
        let status = cmd.status()?;

        if !status.success() {
            return Err(crate::Error::FlutterCli(format!(
                "{} build başarısız oldu",
                platform
            )));
        }

        info!("{} build tamamlandı", platform);
    }

    Ok(())
} 