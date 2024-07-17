use super::{file::ClixFile, ClixPath};

#[derive(Debug)]
pub struct ClixDirectory {
    dir: ClixPath,
    files: Vec<ClixFile>,
    sub_dirs: Vec<ClixDirectory>,
    dir_type: DirectoryType,
}

impl ClixDirectory {
    pub fn new(dir: ClixPath, files: Vec<ClixFile>, sub_dirs: Vec<ClixDirectory>) -> Self {
        ClixDirectory {
            dir: dir.clone(),
            files,
            sub_dirs,
            dir_type: DirectoryType::from_directory_name(dir.name().as_str()),
        }
    }

    pub fn dir_type(&self) -> &DirectoryType {
        &self.dir_type
    }

    pub fn get_command_name(&self) -> String {
        self.dir.name()
    }

    pub fn files(&self) -> &Vec<ClixFile> {
        &self.files
    }

    pub fn sub_dirs(&self) -> &Vec<ClixDirectory> {
        &self.sub_dirs
    }
}

#[derive(Debug)]
pub enum DirectoryType {
    Command,
    Configuration,
}

impl DirectoryType {
    pub fn from_directory_name(name: &str) -> Self {
        if name.starts_with('.') {
            Self::Configuration
        } else {
            Self::Command
        }
    }
}
