use crate::application;
use std::path::PathBuf;

pub struct RelativePath(Vec<String>);

impl RelativePath {
    pub fn new(path: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, ()> {
        let mut components = Vec::new();

        for item in path {
            let s = item.into();

            // 验证组件合法性
            if s.is_empty() || s.contains('/') || s.contains('\\') {
                return Err(());
            }

            // 处理 "." 组件（当前目录）
            if s == "." {
                continue; // 忽略当前目录引用
            }

            // 处理 ".." 组件（上级目录）
            if s == ".." {
                if let Some(last) = components.last() {
                    if last != ".." {
                        // 如果最后一个组件不是 ".."，则移除它（抵消）
                        components.pop();
                        continue;
                    }
                }
                // 如果前面没有可以抵消的组件，则添加 ".."
                components.push(s);
                continue;
            }

            // 添加普通组件
            components.push(s);
        }

        Ok(RelativePath(components))
    }

    pub fn resolve(&self, base_path: impl Into<PathBuf>) -> Result<PathBuf, ()> {
        let mut path = base_path.into();

        for component in &self.0 {
            if component == "." {
                // 当前目录，忽略
                continue;
            } else if component == ".." {
                // 检查是否可以有上级目录
                if path.parent().is_none() {
                    return Err(());
                }
                path.pop();
            } else {
                path.push(component);
            }
        }

        Ok(path)
    }
}

pub enum FileContent {
    FromPath(PathBuf),
    Contents(Vec<u8>),
}

pub enum FileSystem {
    File(RelativePath, FileContent),
    Dir(RelativePath),
}
pub struct FSBundle {
    base_path: PathBuf,
    files: Vec<FileSystem>,
}

pub struct LinkTask {
    from: PathBuf,
    to: PathBuf,
}

pub struct EnvTask {}

pub struct Installer {
    fs_tasks: Vec<FSBundle>,
    link_tasks: Vec<LinkTask>,
    env_tasks: Vec<EnvTask>,
}

impl Installer {
    pub fn new(
        fs_tasks: Vec<FSBundle>,
        link_tasks: Vec<LinkTask>,
        env_tasks: Vec<EnvTask>,
    ) -> Self {
        Self {
            fs_tasks,
            link_tasks,
            env_tasks,
        }
    }

    pub async fn install(self) -> application::Application {
        todo!()
    }
}
