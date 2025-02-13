use clap::Parser;
use flighty::{cli::{Cli, Commands}, commands, Result, i18n};
use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Yapılandırmadan dili yükle veya sistem dilini kullan
    if let Ok(config_file) = flighty::config::ConfigFile::new() {
        if let Ok(config) = config_file.load::<flighty::config::Config>() {
            i18n::set_language(&config.language);
        } else {
            // Yapılandırma dosyası okunamazsa sistem dilini kullan
            let system_lang = i18n::detect_system_language();
            i18n::set_language(&system_lang);
        }
    } else {
        // Yapılandırma dosyası bulunamazsa sistem dilini kullan
        let system_lang = i18n::detect_system_language();
        i18n::set_language(&system_lang);
    }

    // Loglama seviyesini ayarla
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(level.into()))
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_level(true)
        .with_ansi(true)
        .with_writer(std::io::stderr)
        .compact()
        .init();

    match cli.command {
        Commands::Init { path } => {
            commands::init::execute(&path).await?;
        }
        Commands::Config { key, value } => {
            commands::config::execute(key, value)?;
        }
        Commands::Build {
            platform,
            version,
            build_number,
            channel,
        } => {
            commands::build::execute(platform, version, build_number, channel).await?;
        }
        Commands::Deploy { platform, target } => {
            commands::deploy::execute(platform, target).await?;
        }
        Commands::Flutter { command } => {
            commands::flutter::execute(&command).await?;
        }
        Commands::Language { code } => {
            commands::language::execute(code).await?;
        }
    }

    Ok(())
}
