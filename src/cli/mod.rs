use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "flighty")]
#[command(about = "Flutter uygulama dağıtım ve yönetim aracı", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Ayrıntılı çıktı göster
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Yeni bir proje başlat
    Init {
        /// Proje dizini
        #[arg(default_value = ".")]
        path: String,
    },

    /// Yapılandırmayı görüntüle veya düzenle
    Config {
        /// Yapılandırma anahtarı
        key: Option<String>,
        /// Yeni değer
        value: Option<String>,
    },

    /// Uygulamayı derle
    Build {
        /// Derleme platformu (android/ios)
        #[arg(short, long)]
        platform: Option<String>,

        /// Sürüm numarası
        #[arg(short, long)]
        version: Option<String>,

        /// Build numarası
        #[arg(short, long)]
        build_number: Option<u32>,

        /// Release kanalı (alpha/beta/production)
        #[arg(short, long)]
        channel: Option<String>,
    },

    /// Uygulamayı dağıt
    Deploy {
        /// Dağıtım platformu (android/ios)
        #[arg(short, long)]
        platform: Option<String>,

        /// Dağıtım hedefi (store/firebase)
        #[arg(short, long)]
        target: Option<String>,
    },

    /// Flutter SDK'yı yönet
    Flutter {
        /// Flutter komutu
        #[arg(default_value = "doctor")]
        command: String,
    },

    /// Dili değiştir
    Language {
        /// Dil kodu (en/tr)
        #[arg(value_parser = ["en", "tr"])]
        code: String,
    },
} 