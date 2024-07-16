use std::ffi::OsStr;
use std::path::*;

use dir::ClixDirectory;
use file::ClixFile;
use log::debug;
use log::error;
use log::warn;

use crate::execution::CommandFileType;
use crate::execution::FileTypeSpecifier;

pub mod dir;
pub mod file;

// PathBuf wrapper with helper functions
#[derive(Debug, Clone)]
pub struct ClixPath {
    path: PathBuf,
}

impl ClixPath {
    pub fn new(path: PathBuf) -> Self {
        ClixPath { path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn name(&self) -> String {
        String::from(Self::convert_os_string(if self.is_file() {
            self.path.file_stem().expect("could not get file stem")
        } else {
            self.path().file_name().expect("could not get dir name")
        }))
    }

    pub fn extension(&self) -> String {
        Self::convert_os_string(self.path.extension().expect("could not get extension")).to_string()
    }

    pub fn file_type(&self) -> Option<CommandFileType> {
        if self.is_file() {
            Some(CommandFileType::from_extension(self.extension().as_str()))
        } else {
            None
        }
    }

    pub fn get_neighbours_or_contents(&self) -> Vec<PathBuf> {
        if self.is_file() {
            self.path
                .parent()
                .expect("could not get parent of file path")
                .read_dir()
                .expect("could not read parent dir of file path")
                .map(|neighbour| neighbour.expect("could not get dir entry").path())
                .collect()
        } else {
            self.path
                .read_dir()
                .expect("could not read directory")
                .map(|content| content.expect("could not get dir entry").path())
                .collect()
        }
    }

    fn convert_os_string(os_str: &OsStr) -> &str {
        os_str
            .to_str()
            .expect("could not convert os string {os_str:?}")
    }
}

fn read_directory(path: ClixPath) -> ClixDirectory {
    debug!("reading path: {path:?}");
    let mut files: Vec<ClixFile> = Vec::new();
    let mut directories: Vec<ClixDirectory> = Vec::new();

    path.get_neighbours_or_contents().iter().for_each(|entry| {
        let sub_path = ClixPath::new(entry.clone());
        if sub_path.is_file() {
            if let Some(file_type) = sub_path.file_type() {
                match file_type {
                    CommandFileType::Unhandled(unhandled) => {
                        warn!("unhandled file type: {unhandled}")
                    }
                    _ => files.push(ClixFile::new(sub_path)),
                }
            } else {
                error!("file type not determined for file: {sub_path:?}");
            }
        } else {
            directories.push(read_directory(sub_path))
        }
    });

    ClixDirectory::new(path, files, directories)
}

pub fn load_directory(path: PathBuf) -> ClixRepo {
    let root = read_directory(ClixPath::new(path));
    ClixRepo { root }
}

#[derive(Debug)]
pub struct ClixRepo {
    root: ClixDirectory,
}

impl ClixRepo {
    pub(super) fn root_dir(&self) -> &ClixDirectory {
        &self.root
    }
}
