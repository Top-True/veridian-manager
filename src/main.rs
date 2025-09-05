use std::fs;
use veridian_manager::*;

fn main() {
    let config = match fs::read_to_string(dir_path::config().join("config.toml")) {
        Ok(contents) => match config::Config::from_toml(contents.as_str()) {
            Ok(c) => c,
            Err(e) => panic!("Config file parse error:\n, {}", e),
        },
        Err(_) => {
            let c = config::Config::default();
            fs::write(
                dir_path::config().join("config.toml"),
                toml::to_string(&c).unwrap(),
            )
            .unwrap();
            c
        }
    };
    let database = database::Database::new(
        sqlite::Connection::open_thread_safe(dir_path::data().join("database.sqlite")).unwrap(),
    );
}
