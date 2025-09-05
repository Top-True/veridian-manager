use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
static DATA_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn config() -> &'static PathBuf {
    CONFIG_PATH.get_or_init(|| {
        let path = if let Some(p) =
            directories::ProjectDirs::from("top.equaltrue", "", "Veridian Manager")
        {
            p.config_local_dir().to_path_buf()
        } else {
            PathBuf::from("/etc/veridian-manager/config")
        };
        fs::create_dir_all(&path).unwrap();
        path
    })
}

pub fn data() -> &'static PathBuf {
    DATA_PATH.get_or_init(|| {
        let path = if let Some(p) =
            directories::ProjectDirs::from("top.equaltrue", "", "Veridian Manager")
        {
            p.data_local_dir().to_path_buf()
        } else {
            PathBuf::from("/etc/veridian-manager/data")
        };
        fs::create_dir_all(&path).unwrap();
        path
    })
}
