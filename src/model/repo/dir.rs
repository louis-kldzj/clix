use super::{file::ClixFile, ClixPath};

#[derive(Debug)]
pub struct ClixDirectory {
    dir: ClixPath,
    files: Vec<ClixFile>,
    sub_dirs: Vec<ClixDirectory>,
}

impl ClixDirectory {
    pub fn new(dir: ClixPath, files: Vec<ClixFile>, sub_dirs: Vec<ClixDirectory>) -> Self {
        ClixDirectory {
            dir,
            files,
            sub_dirs,
        }
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
