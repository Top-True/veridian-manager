use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

impl Config {
    pub fn from_toml(path: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        let (personal, global) = bundle_deploy::env::get_software_install_paths();
        let personal = Profile {
            default_install_path: PathBuf::from(personal),
        };
        let global = Profile {
            default_install_path: PathBuf::from(global),
        };
        let mut profiles = HashMap::with_capacity(2);
        profiles.insert("personal".to_string(), personal);
        profiles.insert("global".to_string(), global);
        Self { profiles }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "default-install-path")]
    pub default_install_path: PathBuf,
}
