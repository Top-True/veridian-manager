use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();
static DATABASE_CONNECTION: OnceLock<sqlite::ConnectionThreadSafe> = OnceLock::new();

pub fn get_config_path() -> &'static PathBuf {
    CONFIG_FILE_PATH.get_or_init(|| {
        if let Some(p) = directories::ProjectDirs::from("top.equaltrue", "", "Veridian Manager") {
            p.config_dir().to_path_buf()
        } else {
            PathBuf::from("/etc/veridian-manager/veridian-manager")
        }
    })
}

pub fn get_default_toml_config() -> String {
    let (personal, global) = bundle_deploy::env::get_software_install_paths();
    let (personal, global) = (personal.to_string_lossy(), global.to_string_lossy());
    format!(
        include_str!("assets/default-config.toml"),
        default_global_install_path = global,
        default_personal_install_path = personal,
    )
}

pub fn get_database_connection() -> &'static sqlite::ConnectionThreadSafe {
    DATABASE_CONNECTION.get_or_init(|| {
        sqlite::Connection::open_thread_safe(PathBuf::from(get_config_path()).join("db.sqlite")).unwrap()
    })
}

pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

pub struct Profile {
    pub default_install_path: PathBuf,
}
