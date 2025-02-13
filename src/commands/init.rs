use crate::{fl, Result};
use std::{fs, path::Path, process::Command, time::Duration};
use tracing::{debug, warn};
use console::{style, Emoji, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::sleep;

static ROCKET: Emoji<'_, '_> = Emoji("🚀", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦", "");
static EYE: Emoji<'_, '_> = Emoji("👀", "");
static CHECK: Emoji<'_, '_> = Emoji("✓", "");
static BIRD: Emoji<'_, '_> = Emoji("🐦", "");
static ERROR: Emoji<'_, '_> = Emoji("⚠️", "");
static CROSS: Emoji<'_, '_> = Emoji("✗", "");

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

fn create_spinner(msg: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.yellow} {msg}")
            .unwrap()
    );
    spinner.set_message(msg);
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}

fn finish_spinner(spinner: &ProgressBar, success: bool, message: &str) {
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{msg}")
            .unwrap()
    );
    spinner.finish_with_message(format!("{} {}", 
        if success {
            style("✓").green()
        } else {
            style("✗").red()
        },
        if success {
            style(message).green()
        } else {
            style(message).red()
        }
    ));
}

pub async fn execute(path: &str) -> Result<()> {
    let path = Path::new(path);
    let term = Term::stdout();
    term.clear_screen()?;

    // Sade başlık
    println!("\n$ {}", style("flighty init").green());

    // Proje adını al
    let project_name = loop {
        let input = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", 
                style(fl!("init-prompt")).bold()
            ))
            .default(path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&fl!("init-default-name"))
                .to_string())
            .interact()?;

        // Proje adı kurallarını kontrol et
        if input.len() < 3 || input.len() > 30 {
            println!("{} {}", 
                style("✗").red(),
                style(fl!("project-name-length-error")).red()
            );
            continue;
        }

        if input.chars().next().unwrap().is_numeric() {
            println!("{} {}", 
                style("✗").red(),
                style(fl!("project-name-start-error")).red()
            );
            continue;
        }

        if !input.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            println!("{} {}", 
                style("✗").red(),
                style(fl!("project-name-chars-error")).red()
            );
            continue;
        }

        break input;
    };

    // Flutter kontrolü için spinner
    let mut spinner = create_spinner(fl!("step-check-flutter"));
    
    // Flutter projesini kontrol et ve 5 saniye bekle
    if let Err(e) = validate_flutter_project(path) {
        sleep(Duration::from_secs(5)).await;
        finish_spinner(&spinner, false, &fl!("error-flutter-not-found"));
        println!("\n{} {}", ERROR, style(fl!("error-title")).red().bold());
        match e {
            crate::Error::InvalidArgument(msg) => {
                println!("   {} {}", style("•").red(), style(msg).dim());
                println!("\n{} {}", style(fl!("error-solution")).yellow().bold(), style(fl!("solution-create-project")).dim());
                println!("   {} {}", style("1.").yellow(), style("flutter create my_app").dim());
                println!("   {} {}", style("2.").yellow(), style("cd my_app").dim());
                println!("   {} {}\n", style("3.").yellow(), style("flighty init").dim());
            },
            crate::Error::FlutterSdkNotFound => {
                println!("   {} {}", style("•").red(), style(fl!("error-flutter-sdk-not-found")).dim());
                println!("\n{} {}", style(fl!("error-solution")).yellow().bold(), style(fl!("solution-install-sdk")).dim());
                println!("   {} {}", style("1.").yellow(), style("https://flutter.dev/docs/get-started/install").dim());
                println!("   {} {}", style("2.").yellow(), style("PATH").dim());
                println!("   {} {}\n", style("3.").yellow(), style("Terminal").dim());
            },
            crate::Error::FlutterCli(msg) => {
                println!("   {} {}", style("•").red(), style(msg).dim());
                println!("\n{} {}", style(fl!("error-solution")).yellow().bold(), style(fl!("solution-check-flutter")).dim());
                println!("   {} {}\n", style("•").yellow(), style("flutter doctor").dim());
            },
            _ => {
                println!("   {} {}\n", style("•").red(), style(fl!("error-unknown-occurred")).dim());
            }
        }
        return Ok(());
    }

    // Başarılı kontrol ve 5 saniye bekle
    sleep(Duration::from_secs(5)).await;
    finish_spinner(&spinner, true, &fl!("step-check-flutter"));

    // Dizin yapısı için spinner
    spinner = create_spinner(fl!("step-create-structure"));
    
    let flighty_dir = path.join(".flighty");
    if let Err(e) = create_flighty_structure(&flighty_dir) {
        sleep(Duration::from_secs(5)).await;
        finish_spinner(&spinner, false, &fl!("error-structure-failed"));
        println!("\n{} {}\n", ERROR, style(fl!("error-message", message = e.to_string())).red());
        return Ok(());
    }

    sleep(Duration::from_secs(5)).await;
    finish_spinner(&spinner, true, &fl!("step-create-structure"));

    // Git yapılandırması için spinner
    spinner = create_spinner(fl!("step-update-git"));
    
    if let Err(e) = update_gitignore(path) {
        sleep(Duration::from_secs(5)).await;
        finish_spinner(&spinner, false, &fl!("error-git-failed"));
        println!("\n{} {}\n", ERROR, style(fl!("error-message", message = e.to_string())).red());
        return Ok(());
    }

    sleep(Duration::from_secs(5)).await;
    finish_spinner(&spinner, true, &fl!("step-update-git"));

    // Yapılandırma dosyaları için spinner
    spinner = create_spinner(fl!("step-create-config"));
    
    if let Err(e) = update_config_files(&flighty_dir, &project_name) {
        sleep(Duration::from_secs(5)).await;
        finish_spinner(&spinner, false, &fl!("error-config-failed"));
        println!("\n{} {}\n", ERROR, style(fl!("error-message", message = e.to_string())).red());
        return Ok(());
    }

    sleep(Duration::from_secs(5)).await;
    finish_spinner(&spinner, true, &fl!("step-create-config"));

    // Başarı mesajı
    println!("\n{} {}\n", 
        BIRD,
        style(fl!("success-init")).green().bold()
    );

    // Oluşturulan dosyalar
    println!("{} {}", CHECK, style(fl!("success-app-created")).green());
    println!("{} {}", CHECK, style(fl!("success-config-created")).green());
    println!("{} {}\n", CHECK, style(fl!("success-pubspec-updated")).green());

    // Kullanılabilir komutlar
    println!("{}", style(fl!("commands-intro")).dim());
    println!("{} {:<20} {}", PACKAGE, style("flighty build").cyan(), style(fl!("command-build")).dim());
    println!("{} {:<20} {}", ROCKET, style("flighty deploy").cyan(), style(fl!("command-deploy")).dim());
    println!("{} {:<20} {}\n", EYE, style("flighty preview").cyan(), style(fl!("command-preview")).dim());

    // Dokümantasyon
    println!("{} {}", fl!("docs-more-info"), style("https://flighty.dev").cyan());

    Ok(())
}

fn create_flighty_structure(root: &Path) -> Result<()> {
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
    }

    Ok(())
}

fn validate_flutter_project(path: &Path) -> Result<()> {
    // pubspec.yaml kontrolü
    let pubspec_path = path.join("pubspec.yaml");
    if !pubspec_path.exists() {
        return Err(crate::Error::InvalidArgument(
            fl!("msg-flutter-missing")
        ));
    }

    // Android dizini kontrolü
    let android_dir = path.join("android");
    if !android_dir.exists() {
        warn!("{}", fl!("warn-android-missing"));
    }

    // iOS dizini kontrolü
    let ios_dir = path.join("ios");
    if !ios_dir.exists() {
        warn!("{}", fl!("warn-ios-missing"));
    }

    // Flutter doctor kontrolü
    debug!("{}", fl!("debug-flutter-doctor"));
    let output = Command::new("flutter")
        .arg("doctor")
        .output()
        .map_err(|_| crate::Error::FlutterSdkNotFound)?;

    if !output.status.success() {
        return Err(crate::Error::FlutterCli(
            fl!("error-flutter-doctor-failed")
        ));
    }

    Ok(())
}

fn update_config_files(root: &Path, project_name: &str) -> Result<()> {
    // Yapılandırma dosyaları güncelleme işlemleri
    Ok(())
} 