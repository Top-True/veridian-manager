use clap::Parser;
use rhai::Engine;
use std::fs;
use std::path::{Path, PathBuf};
use veridian_manager::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to a script file
    #[arg(short, long)]
    script: PathBuf,
}

fn main() {
    let args = Args::parse();
    let config = match fs::read_to_string(dir_path::config().join("config.toml")) {
        Ok(contents) => match config::Config::from_toml(contents.as_str()) {
            Ok(c) => c,
            Err(e) => occur_error("Config File Parse Error", e),
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
    let builder = match create_builder_from_script(args.script.as_path()) {
        Ok(b) => b,
        Err(e) => occur_error("Script Error", e),
    };
    println!("{:#?}", builder);
}

fn create_builder_from_script(
    path: &Path,
) -> Result<installer_builder::InstallerBuilder, Box<rhai::EvalAltResult>> {
    let mut engine = Engine::new();
    engine
        .register_type::<installer_builder::InstallerBuilder>()
        .register_fn("InstallerBuilder", installer_builder::InstallerBuilder::new);
    engine.eval_file::<installer_builder::InstallerBuilder>(path.to_path_buf())
}

fn occur_error(title: &str, error: impl std::error::Error) -> ! {
    eprintln!("{}:", title);
    eprintln!("{}\n", error);
    std::process::exit(1);
}
