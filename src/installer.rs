use crate::{application, recorder};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateDirectoryTask(PathBuf);

impl CreateDirectoryTask {
    pub fn new(path: PathBuf) -> Self {
        CreateDirectoryTask(path)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }
}

#[derive(Debug)]
pub enum WriteFileTask {
    FromPath { from: PathBuf, to: PathBuf },
    Contents { content: Vec<u8>, to: PathBuf },
}

#[derive(Debug)]
pub enum LinkType {
    Shortcut,
    Symbolic,
    Hard,
}

#[derive(Debug)]
pub struct CreateLinkTask {
    from: PathBuf,
    to: PathBuf,
    link_type: LinkType,
}

#[derive(Debug)]
pub struct EnvTask {}

pub struct Installer {
    dir_tasks: Vec<CreateDirectoryTask>,
    file_tasks: Vec<WriteFileTask>,
    link_tasks: Vec<CreateLinkTask>,
    env_tasks: Vec<EnvTask>,
}

impl Installer {
    pub fn new(
        dir_tasks: Vec<CreateDirectoryTask>,
        file_tasks: Vec<WriteFileTask>,
        link_tasks: Vec<CreateLinkTask>,
        env_tasks: Vec<EnvTask>,
    ) -> Self {
        Self {
            dir_tasks,
            file_tasks,
            link_tasks,
            env_tasks,
        }
    }

    pub async fn install(self) -> InstallResult {
        let mut recorder = recorder::Recorder::with_capacity(
            self.dir_tasks.len(),
            self.file_tasks.len(),
            self.link_tasks.len(),
            self.env_tasks.len(),
        );
        for task in self.dir_tasks {
            let res = bundle_deploy::file_system::create_dir_all(task.path())
                .await;
            match res {
                Ok(_) => recorder.record_directory(recorder::DirectoryRecord::from(task.path().clone())),
                Err(_) => return Err((recorder, InstallErr::CreateDirectory(task.path().clone()))),
            }
        }
        for task in self.file_tasks {
            match task {
                WriteFileTask::FromPath { from, to } => {
                    bundle_deploy::file_system::copy(from, &to).await.unwrap(); // todo
                    recorder.record_file(recorder::FileRecord::from(to));
                }
                WriteFileTask::Contents { content, to } => {
                    bundle_deploy::file_system::write(&to, content)
                        .await
                        .unwrap(); // todo
                    recorder.record_file(recorder::FileRecord::from(to));
                }
            }
        }
        // todo!()
        Ok(application::Application::from(recorder))
    }
}

pub enum InstallErr {
    CreateDirectory(PathBuf),
    WriteFile,
    CreateLink,
    Env,
}

pub type InstallResult = Result<application::Application, (recorder::Recorder, InstallErr)>;
