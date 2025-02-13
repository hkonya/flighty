use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_DIR_NAME: &str = ".flighty";
const CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Debug)]
pub struct ConfigFile {
    path: PathBuf,
}

impl ConfigFile {
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        Self::find_in_ancestors(&current_dir)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    fn find_in_ancestors(start_dir: &Path) -> Result<Self> {
        let mut current_dir = start_dir.to_path_buf();
        loop {
            let config_dir = current_dir.join(CONFIG_DIR_NAME);
            let config_path = config_dir.join(CONFIG_FILE_NAME);
            if config_path.exists() {
                return Ok(Self::from_path(config_path));
            }

            if !current_dir.pop() {
                return Err(Error::ProjectConfigNotFound);
            }
        }
    }

    pub fn load<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = fs::read_to_string(&self.path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }

    pub fn save<T: Serialize>(&self, config: &T) -> Result<()> {
        // Dizinin var olduğundan emin ol
        if let Some(dir) = self.path.parent() {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }

        let contents = serde_yaml::to_string(config)?;
        fs::write(&self.path, contents)?;
        Ok(())
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn create_default<T: Serialize + Default>(&self) -> Result<T> {
        let config = T::default();
        self.save(&config)?;
        Ok(config)
    }
} 