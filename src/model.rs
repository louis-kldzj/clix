use std::ffi::OsStr;
use std::fs::*;
use std::path::*;

use clap::ArgMatches;
use clap::Command;
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

    fn convert_os_string(os_str: &OsStr) -> &str {
        os_str
            .to_str()
            .expect("could not convert os string {os_str:?}")
    }
}

#[derive(Debug)]
pub struct ClixFile {
    pub(super) file: ClixPath,
}

impl ClixFile {
    pub fn new(file: ClixPath) -> Self {
        ClixFile { file }
    }

    pub fn from_dir_entry(dir_entry: DirEntry) -> Self {
        ClixFile {
            file: ClixPath::new(dir_entry.path()),
        }
    }

    pub fn get_file_name(&self) -> String {
        self.file.name()
    }
}

#[derive(Debug)]
pub struct ClixDirectory {
    pub(super) dir: ClixPath,
    pub(super) files: Vec<ClixFile>,
    pub(super) sub_dirs: Vec<ClixDirectory>,
}

impl ClixDirectory {
    pub fn get_command_name(&self) -> String {
        self.dir.name()
    }
}

fn read_path_buf(path: PathBuf) -> ClixDirectory {
    debug!("reading path: {path:?}");
    let mut files: Vec<ClixFile> = Vec::new();
    let mut directories: Vec<ClixDirectory> = Vec::new();

    path.read_dir()
        .expect("cannot read directory")
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    let last_path_cmp = get_last_path_component_as_string(entry.path());
                    if !last_path_cmp.starts_with('.') {
                        if file_type.is_dir() {
                            directories.push(read_path_buf(entry.path()));
                        } else {
                            files.push(ClixFile::from_dir_entry(entry));
                        }
                    }
                }
            }
        });

    ClixDirectory {
        dir: ClixPath::new(path),
        files,
        sub_dirs: directories,
    }
}

pub fn load_directory() -> ClixRepo {
    const DIR: &str = "/home/locuris/code/clix/test-repo/engage";
    let path = PathBuf::from(DIR);
    let root = read_path_buf(path);
    ClixRepo::new(root)
}

fn create_command(dir: &ClixDirectory) -> Command {
    let mut command = Command::new(dir.get_command_name());
    for file in &dir.files {
        command = command.subcommand(Command::new(file.get_file_name()));
    }
    for sub_dir in &dir.sub_dirs {
        command = command.subcommand(create_command(sub_dir));
    }
    command
}

#[derive(Debug)]
pub struct ClixRepo {
    root: ClixDirectory,
}

impl ClixRepo {
    pub(super) fn new(root: ClixDirectory) -> Self {
        ClixRepo { root }
    }

    pub fn clap(&self) -> Command {
        create_command(&self.root)
    }

    pub fn clap_file(&self) -> Option<ClixFile> {
        debug!("clapping file");
        let matches = self.clap().get_matches();
        Self::walk_repo(&matches, &self.root)
    }

    fn walk_repo(arg_match: &ArgMatches, clix_dir: &ClixDirectory) -> Option<ClixFile> {
        let command_name = arg_match.subcommand_name().unwrap();
        let dir_name = clix_dir.get_command_name();
        debug!("walking repo for {command_name} in {dir_name}");
        if let Some((cmd_name, next_match)) = arg_match.subcommand() {
            debug!("got subcommand {cmd_name}");
            for dir in &clix_dir.sub_dirs {
                if dir.get_command_name() == cmd_name {
                    return Self::walk_repo(next_match, dir);
                }
            }
        }
        debug!("should be on last command...");
        for file in &clix_dir.files {
            debug!("checking file: {file:?}");
            if file.get_file_name()
                == arg_match
                    .subcommand_name()
                    .expect("could not get subcommand name")
            {
                return file.get_file();
            }
        }

        None
    }
}

// Helper Method
fn get_last_path_component_as_string(path: PathBuf) -> String {
    String::from(
        path.components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
    )
}
