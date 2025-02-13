use crate::Result;
use std::process::Command;
use tracing::{debug, info};
use which::which;

pub async fn execute(command: &str) -> Result<()> {
    // Flutter CLI'ı kontrol et
    let flutter_path = which("flutter").map_err(|_| crate::Error::FlutterSdkNotFound)?;
    debug!("Flutter yolu: {}", flutter_path.display());

    // Komutu çalıştır
    info!("Flutter komutu çalıştırılıyor: {}", command);
    let mut cmd = Command::new(flutter_path);
    cmd.arg(command);

    debug!("Çalıştırılacak komut: {:?}", cmd);
    let status = cmd.status()?;

    if !status.success() {
        return Err(crate::Error::FlutterCli(format!(
            "Flutter komutu başarısız oldu: {}",
            command
        )));
    }

    Ok(())
} 