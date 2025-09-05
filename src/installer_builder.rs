use crate::installer;
use bundle_deploy::file_system::{FileName, RelativePath};
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::PathBuf;

pub enum SourcePath {
    Disk(PathBuf, glob::Pattern),
    Archive(PathBuf, RelativePath, glob::Pattern),
}

pub struct Source {
    pub path: SourcePath,
    pub destination: PathBuf,
}

#[inline]
fn resolve_stack_util(stack: &Vec<VecDeque<FileName>>) -> RelativePath {
    let mut path = Vec::new();
    for d in stack {
        path.push(d.get(0).unwrap().clone());
    }
    RelativePath::new(path).unwrap()
}

impl Source {
    pub fn resolve(&self) -> SourceResolveResult {
        let mut dir_tasks = Vec::with_capacity(64);
        let mut file_tasks = Vec::with_capacity(128);
        match &self.path {
            SourcePath::Disk(abs, pat) => {
                let mut stack = Vec::<VecDeque<FileName>>::with_capacity(128);
                'a: loop {
                    let read_dir =
                        match fs::read_dir(resolve_stack_util(&stack).resolve(abs).unwrap()) {
                            Ok(read_dir) => read_dir,
                            Err(e) => return Err(SourceResolveErr::ReadDirErr(e)),
                        };
                    let mut dir_deque = VecDeque::new();
                    for entry in read_dir {
                        let entry = match entry {
                            Ok(entry) => entry,
                            Err(e) => return Err(SourceResolveErr::ReadDirErr(e)),
                        };
                        let path = entry.path();
                        if !pat.matches_path(&path) {
                            continue;
                        }
                        let mut relative_path = resolve_stack_util(&stack);
                        relative_path.push(FileName::new(entry.file_name()).unwrap());
                        if path.is_file() {
                            file_tasks.push(installer::WriteFileTask::FromPath {
                                from: path,
                                to: relative_path.resolve(&self.destination).unwrap(),
                            });
                        } else if path.is_dir() {
                            dir_tasks.push(installer::CreateDirectoryTask::new(
                                relative_path.resolve(&self.destination).unwrap(),
                            ));
                            dir_deque.push_back(FileName::new(entry.file_name()).unwrap());
                        }
                    }
                    loop {
                        if !dir_deque.is_empty() {
                            stack.push(dir_deque);
                            break;
                        } else {
                            dir_deque = match stack.pop() {
                                Some(dir_deque) => dir_deque,
                                None => break 'a,
                            };
                            dir_deque.pop_front();
                        }
                    }
                }
            }
            SourcePath::Archive(_, _, _) => {
                todo!()
            }
        }
        Ok(SourceResolveOK {
            dir_tasks,
            file_tasks,
        })
    }
}

#[derive(Debug)]
pub struct SourceResolveOK {
    pub dir_tasks: Vec<installer::CreateDirectoryTask>,
    pub file_tasks: Vec<installer::WriteFileTask>,
}

#[derive(Debug)]
pub enum SourceResolveErr {
    ReadDirErr(std::io::Error),
}

impl Display for SourceResolveErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for SourceResolveErr {}

pub type SourceResolveResult = Result<SourceResolveOK, SourceResolveErr>;

pub struct InstallerBuilder {
    sources: Vec<Source>,
    dir_sources: Vec<PathBuf>,
}

impl InstallerBuilder {
    pub fn new() -> Self {
        Self {
            sources: Vec::with_capacity(5),
            dir_sources: Vec::with_capacity(5),
        }
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    pub fn build(self) -> BuildResult {
        let mut dir_tasks = Vec::with_capacity(self.dir_sources.len() + self.sources.len() * 32);
        let mut file_tasks = Vec::with_capacity(128);
        let mut link_tasks = Vec::with_capacity(5);
        let mut env_tasks = Vec::with_capacity(5);
        for dir_source in self.dir_sources {
            dir_tasks.push(installer::CreateDirectoryTask::new(dir_source));
        }
        for source in self.sources {
            let result = source.resolve();
            let (mut source_dir_tasks, mut source_file_tasks) = match result {
                Ok(r) => (r.dir_tasks, r.file_tasks),
                Err(e) => return Err(BuildError::SourceError(e)),
            };
            dir_tasks.append(&mut source_dir_tasks);
            file_tasks.append(&mut source_file_tasks);
        }
        // todo!()
        Ok(installer::Installer::new(
            dir_tasks, file_tasks, link_tasks, env_tasks,
        ))
    }
}

#[derive(Debug)]
pub enum BuildError {
    PatternError(glob::PatternError),
    GlobError(glob::GlobError),
    SourceError(SourceResolveErr),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for BuildError {}

pub type BuildResult = Result<installer::Installer, BuildError>;
