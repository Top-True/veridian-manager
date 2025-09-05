use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct DirectoryRecord(PathBuf);

impl From<PathBuf> for DirectoryRecord {
    fn from(path: PathBuf) -> Self {
        DirectoryRecord(path)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileRecord(PathBuf);

impl From<PathBuf> for FileRecord {
    fn from(path: PathBuf) -> Self {
        FileRecord(path)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LinkRecord(PathBuf);

impl From<PathBuf> for LinkRecord {
    fn from(path: PathBuf) -> Self {
        LinkRecord(path)
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnvRecord {}

#[derive(Serialize, Deserialize)]
pub struct Recorder {
    dir_tasks: Vec<DirectoryRecord>,
    file_tasks: Vec<FileRecord>,
    link_tasks: Vec<LinkRecord>,
    env_tasks: Vec<EnvRecord>,
}

impl Recorder {
    pub fn with_capacity(dir: usize, file: usize, link: usize, env: usize) -> Self {
        Self {
            dir_tasks: Vec::with_capacity(dir),
            file_tasks: Vec::with_capacity(file),
            link_tasks: Vec::with_capacity(link),
            env_tasks: Vec::with_capacity(env),
        }
    }

    pub fn record_directory(&mut self, record: DirectoryRecord) {
        self.dir_tasks.push(record);
    }

    pub fn record_file(&mut self, record: FileRecord) {
        self.file_tasks.push(record);
    }

    pub fn record_link(&mut self, record: LinkRecord) {
        self.link_tasks.push(record);
    }

    pub fn record_env(&mut self, record: EnvRecord) {
        self.env_tasks.push(record);
    }
}

impl Recorder {
    pub async fn rollback(self) -> Result<(), ()> {
        todo!()
    }
}

impl Recorder {
    pub fn to_binary(&self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }

    pub fn from_binary(data: &[u8]) -> Self {
        bincode::serde::decode_from_slice(data, bincode::config::standard())
            .unwrap()
            .0
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            dir_tasks: Vec::new(),
            file_tasks: Vec::new(),
            link_tasks: Vec::new(),
            env_tasks: Vec::new(),
        }
    }
}
