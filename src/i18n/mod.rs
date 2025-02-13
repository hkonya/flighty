use fluent::{FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;
use tracing::debug;

// Desteklenen diller
pub const SUPPORTED_LANGUAGES: &[&str] = &["en", "tr"];
const DEFAULT_LANGUAGE: &str = "en";

// fl! makrosu
#[macro_export]
macro_rules! fl {
    ($key:expr) => {
        $crate::i18n::translate($key)
    };
    ($key:expr, $($k:ident = $v:expr),*) => {{
        let mut args = ::std::collections::HashMap::new();
        $(
            args.insert(stringify!($k), $v.to_string());
        )*
        $crate::i18n::translate_with_args($key, Some(&args))
    }};
}

// Global dil yöneticisi
static I18N: Lazy<Mutex<I18nManager>> = Lazy::new(|| {
    let manager = I18nManager::new();
    Mutex::new(manager)
});

pub struct I18nManager {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    current_language: String,
}

unsafe impl Send for I18nManager {}
unsafe impl Sync for I18nManager {}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

impl I18nManager {
    fn new() -> Self {
        let mut manager = Self {
            bundles: HashMap::new(),
            current_language: detect_system_language(),
        };

        // Dil dosyalarını yükle
        for lang in SUPPORTED_LANGUAGES {
            if let Ok(bundle) = manager.create_bundle(lang) {
                manager.bundles.insert(lang.to_string(), bundle);
            }
        }

        manager
    }

    fn create_bundle(&self, lang_code: &str) -> anyhow::Result<FluentBundle<FluentResource>> {
        let mut bundle = FluentBundle::new(vec![lang_code.parse()?]);
        
        // Dil dosyası yolunu belirle
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().ok_or_else(|| anyhow::anyhow!("Executable directory not found"))?;
        let path = exe_dir.join("locales").join(lang_code).join("main.ftl");

        debug!("Dil dosyası yolu: {}", path.display());
        
        // Dosyayı oku
        let source = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                debug!("Dil dosyası okunamadı: {} - {}", path.display(), e);
                // Geliştirme ortamında deneyelim
                let dev_path = PathBuf::from(format!("locales/{}/main.ftl", lang_code));
                fs::read_to_string(&dev_path).map_err(|e| {
                    debug!("Geliştirme dil dosyası da okunamadı: {} - {}", dev_path.display(), e);
                    anyhow::anyhow!("Failed to read language file: {}", e)
                })?
            }
        };

        // FTL dosyasını ayrıştır
        let resource = FluentResource::try_new(source)
            .map_err(|(_resource, errors)| {
                debug!("FTL ayrıştırma hatası: {:?}", errors);
                anyhow::anyhow!("Failed to parse FTL: {:?}", errors)
            })?;

        // Bundle'a ekle
        bundle.add_resource(resource)
            .map_err(|errors| {
                debug!("Bundle ekleme hatası: {:?}", errors);
                anyhow::anyhow!("Failed to add FTL resource: {:?}", errors)
            })?;

        Ok(bundle)
    }

    fn get_message(&self, key: &str, args: Option<&HashMap<&str, String>>) -> String {
        if let Some(bundle) = self.bundles.get(&self.current_language) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let mut args_list = fluent::FluentArgs::new();
                    if let Some(args) = args {
                        for (k, v) in args {
                            args_list.set(*k, v.as_str());
                        }
                    }
                    return bundle
                        .format_pattern(pattern, Some(&args_list), &mut errors)
                        .into_owned();
                }
            }
            debug!("Mesaj bulunamadı: {}", key);
        } else {
            debug!("Bundle bulunamadı: {}", self.current_language);
        }
        key.to_string()
    }
}

// Public API
pub fn set_language(lang_code: &str) -> bool {
    if SUPPORTED_LANGUAGES.contains(&lang_code) {
        if let Ok(mut manager) = I18N.lock() {
            manager.current_language = lang_code.to_string();
            return true;
        }
    }
    false
}

pub fn get_current_language() -> String {
    I18N.lock()
        .map(|manager| manager.current_language.clone())
        .unwrap_or_else(|_| DEFAULT_LANGUAGE.to_string())
}

pub fn translate(key: &str) -> String {
    translate_with_args(key, None)
}

pub fn translate_with_args(key: &str, args: Option<&HashMap<&str, String>>) -> String {
    I18N.lock()
        .map(|manager| manager.get_message(key, args))
        .unwrap_or_else(|_| key.to_string())
}

// Dil kodunu sistem diline göre belirle
pub fn detect_system_language() -> String {
    // Sistem dilini al
    if let Some(locale) = get_locale() {
        // Dil kodunu ayır (örn: "tr-TR" -> "tr")
        let lang_code = locale.split(['-', '_']).next().unwrap_or(DEFAULT_LANGUAGE);
        
        // Desteklenen diller arasında var mı kontrol et
        if SUPPORTED_LANGUAGES.contains(&lang_code) {
            return lang_code.to_string();
        }

        // Daha detaylı eşleştirme için fluent-langneg kullan
        let requested = vec![locale.as_str()];
        let available = SUPPORTED_LANGUAGES
            .iter()
            .map(|s| s.parse::<LanguageIdentifier>().unwrap())
            .collect::<Vec<_>>();

        let requested = requested
            .iter()
            .filter_map(|s| s.parse::<LanguageIdentifier>().ok())
            .collect::<Vec<_>>();

        let default = DEFAULT_LANGUAGE.parse::<LanguageIdentifier>().unwrap();
        let selected = negotiate_languages(
            &requested,
            &available,
            Some(&default),
            NegotiationStrategy::Filtering,
        );

        return selected[0].language.to_string();
    }

    // Sistem dili alınamazsa varsayılan dili kullan
    DEFAULT_LANGUAGE.to_string()
} 