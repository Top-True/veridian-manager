use directories::BaseDirs;
use std::env;
use std::path::PathBuf;

pub fn get_software_install_paths() -> (PathBuf, PathBuf) {
    let user_path = BaseDirs::new().map_or("/".into(), |dirs| dirs.data_local_dir().into());
    let global_path = if cfg!(target_os = "windows") {
        env::var("ProgramFiles").map_or("/".into(), |p| p.into())
    } else if cfg!(target_os = "linux") {
        "/usr/bin".into()
    } else if cfg!(target_os = "macos") {
        "/Applications".into()
    } else {
        "/".into()
    };
    (user_path, global_path)
}
