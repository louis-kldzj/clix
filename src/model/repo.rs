use std::ffi::OsStr;
use std::path::*;

use log::debug;

use crate::config::get_command_configuration;
use crate::config::CommandConfiguration;

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

#[derive(Debug, Clone)]
pub struct ClixFile {
    file: ClixPath,
    config_file: Option<ClixPath>,
}

impl ClixFile {
    pub fn new(file: ClixPath) -> Self {
        let mut clix = ClixFile {
            file,
            config_file: None,
        };
        clix.try_set_config_file();
        clix
    }

    pub fn file_path(&self) -> &ClixPath {
        &self.file
    }

    pub fn get_file_name(&self) -> String {
        self.file.name()
    }

    pub fn get_config(&self) -> Option<CommandConfiguration> {
        if let Some(config_file) = &self.config_file {
            if let Ok(config) = get_command_configuration(config_file.path().clone()) {
                return Some(config);
            }
        }
        None
    }

    fn try_set_config_file(&mut self) {
        let file_name = self.file.name();
        for neighbour in self.file.get_neighbours_or_contents() {
            if !neighbour.is_dir() {
                let path = ClixPath::new(neighbour);
                let name = path.name();
                if name != file_name && name.contains(file_name.as_str()) {
                    self.config_file = Some(path);
                    return;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ClixDirectory {
    dir: ClixPath,
    files: Vec<ClixFile>,
    sub_dirs: Vec<ClixDirectory>,
}

impl ClixDirectory {
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

fn read_directory(path: ClixPath) -> ClixDirectory {
    debug!("reading path: {path:?}");
    let mut files: Vec<ClixFile> = Vec::new();
    let mut directories: Vec<ClixDirectory> = Vec::new();

    path.get_neighbours_or_contents().iter().for_each(|entry| {
        let sub_path = ClixPath::new(entry.clone());
        if sub_path.is_file() {
            files.push(ClixFile::new(sub_path));
        } else {
            directories.push(read_directory(sub_path))
        }
    });

    ClixDirectory {
        dir: path,
        files,
        sub_dirs: directories,
    }
}

pub fn load_directory() -> ClixRepo {
    const DIR: &str = "/home/locuris/code/clix/test-repo/engage";
    let path = PathBuf::from(DIR);
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
