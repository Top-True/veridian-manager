use std::path::PathBuf;
use uuid::Uuid;

pub struct Link {}
pub struct Env {}
pub struct Directory(PathBuf);

impl Directory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Directory(path.into())
    }

    pub async fn delete(self) -> Result<(), ()> {
        todo!()
    }
}

pub struct File(PathBuf);

impl File {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        File(path.into())
    }

    pub async fn delete(self) -> Result<(), ()> {
        todo!()
    }
}

pub struct Application {
    id: Uuid,
    dirs: Vec<Directory>,
    files: Vec<File>,
    links: Vec<Link>,
    envs: Vec<Env>,
}

impl Application {
    pub async fn from_database(id: Uuid) -> Self {
        todo!()
    }

    pub async fn uninstall(self) -> Result<(), ()> {
        todo!()
    }
}
