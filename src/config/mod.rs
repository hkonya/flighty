use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod file;
pub use file::ConfigFile;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub flutter: FlutterConfig,
    
    #[serde(default)]
    pub android: AndroidConfig,
    
    #[serde(default)]
    pub ios: IosConfig,
    
    #[serde(default)]
    pub build: BuildConfig,

    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlutterConfig {
    pub sdk_path: Option<PathBuf>,
    pub version: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AndroidConfig {
    pub keystore_path: Option<PathBuf>,
    pub keystore_password: Option<String>,
    pub key_alias: Option<String>,
    pub key_password: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IosConfig {
    pub bundle_id: Option<String>,
    pub team_id: Option<String>,
    pub provisioning_profile: Option<PathBuf>,
    pub certificate: Option<PathBuf>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    pub release_channel: ReleaseChannel,
    pub version_strategy: VersionStrategy,
    pub build_number: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseChannel {
    Alpha,
    Beta,
    Production,
}

impl Default for ReleaseChannel {
    fn default() -> Self {
        Self::Alpha
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionStrategy {
    Semver,
    Timestamp,
    Custom(String),
}

impl Default for VersionStrategy {
    fn default() -> Self {
        Self::Semver
    }
} 