use std::ffi::OsString;
use std::path::PathBuf;
pub use tokio::fs::*;

#[derive(Debug, Clone)]
pub struct FileName(OsString);

impl FileName {
    pub fn new(s: OsString) -> Result<Self, ()> {
        if s.is_empty() {
            return Err(());
        }
        if Self::contains_illegal_chars(&s) {
            return Err(());
        }
        Ok(Self(s))
    }

    pub fn file_name(&self) -> OsString {
        self.0.clone()
    }

    pub fn contains_illegal_chars(os_string: &OsString) -> bool {
        // 首先尝试转换为 UTF-8 字符串
        if let Some(s) = os_string.to_str() {
            s.contains('/') || s.contains('\\')
        } else {
            // 如果非 UTF-8，使用平台特定方法
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStrExt;
                let bytes = os_string.as_os_str().as_bytes();
                bytes.contains(&b'/') || bytes.contains(&b'\\')
            }
            #[cfg(windows)]
            {
                use std::os::windows::ffi::OsStrExt;
                let mut wide_chars = os_string.as_os_str().encode_wide();
                wide_chars.any(|c| c == 47 || c == 92) // 47 是 '/'，92 是 '\' 的代码点
            }
            // 对于其他平台（如 wasm），可能需要不同处理，这里默认返回 false
            #[cfg(not(any(unix, windows)))]
            {
                false // 或其他逻辑，但通常这些平台没有路径分隔符限制
            }
        }
    }
}

impl Into<OsString> for FileName {
    fn into(self) -> OsString {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum RelativePathComponent {
    Super,
    Component(FileName),
}

impl From<FileName> for RelativePathComponent {
    fn from(file_name: FileName) -> Self {
        RelativePathComponent::Component(file_name)
    }
}

#[derive(Debug, Clone)]
pub struct RelativePath(Vec<RelativePathComponent>);

impl RelativePath {
    pub fn new(path: impl IntoIterator<Item = impl Into<OsString>>) -> Result<Self, ()> {
        let mut components = Vec::<RelativePathComponent>::new();

        for item in path {
            let s = item.into();

            // 处理 "." 组件（当前目录）
            if s == "." {
                continue; // 忽略当前目录引用
            }

            // 处理 ".." 组件（上级目录）
            if s == ".." {
                if let Some(last) = components.last() {
                    if matches!(last, RelativePathComponent::Component(_)) {
                        // 如果最后一个组件不是 ".."，则移除它（抵消）
                        components.pop();
                        continue;
                    }
                }
                // 如果前面没有可以抵消的组件，则添加 ".."
                components.push(RelativePathComponent::Super);
                continue;
            }

            // 添加普通组件
            let s = match crate::file_system::FileName::new(s) {
                Ok(n) => n,
                Err(_) => return Err(()),
            };
            components.push(RelativePathComponent::Component(s));
        }

        Ok(RelativePath(components))
    }

    pub fn push(&mut self, component: impl Into<RelativePathComponent>) {
        let component = component.into();
        match component {
            RelativePathComponent::Super => {
                if let Some(last) = self.0.last() {
                    if matches!(last, RelativePathComponent::Component(_)) {
                        // 如果最后一个组件不是 ".."，则移除它（抵消）
                        self.0.pop();
                    }
                }
                // 如果前面没有可以抵消的组件，则添加 ".."
                self.0.push(RelativePathComponent::Super);
            }
            RelativePathComponent::Component(_) => {
                self.0.push(component);
            }
        }
    }

    pub fn resolve(&self, base_path: impl Into<PathBuf>) -> Result<PathBuf, ()> {
        let mut path = base_path.into();
        for component in &self.0 {
            match component {
                RelativePathComponent::Super => {
                    if path.parent().is_none() {
                        return Err(());
                    }
                    path.pop();
                }
                RelativePathComponent::Component(file_name) => {
                    path.push(file_name.file_name());
                }
            }
        }
        Ok(path)
    }
}

impl From<FileName> for RelativePath {
    fn from(name: FileName) -> Self {
        Self(vec![RelativePathComponent::Component(name)])
    }
}
