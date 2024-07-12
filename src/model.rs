use std::fs::*;
use std::path::*;

use clap::ArgMatches;
use clap::Command;
use log::debug;

use crate::config::get_command_configuration;
use crate::config::CommandConfiguration;

pub enum ClixObject {
    CommandDirectory,
    CommandFile,
}

#[derive(Debug)]
pub struct ClixFile {
    pub(super) file: DirEntry,
    config_file: Option<DirEntry>,
}

impl ClixFile {
    pub fn new(file: DirEntry) -> Self {
        ClixFile {
            file,
            config_file: None,
        }
    }

    pub fn new_with_config(file: DirEntry, config_file: Option<DirEntry>) -> Self {
        ClixFile { file, config_file }
    }

    pub fn get_command_name(&self) -> String {
        String::from(
            self.file
                .path()
                .file_stem()
                .expect("could not read file stem")
                .to_str()
                .expect("could not convert file stem to str"),
        )
    }

    pub fn get_config_file(&self) -> Option<CommandConfiguration> {
        let config = &self.config_file;
        config.map(|file| get_command_configuration(&file).expect("could not get config file"))
    }

    pub fn get_file(&self) -> Option<Self> {
        let mut file: Option<DirEntry> = None;
        let mut config_file: Option<DirEntry> = None;
        for ele in self
            .file
            .path()
            .parent()
            .expect("could not get parent path of file")
            .read_dir()
            .expect("could not read parent directory")
        {
            let Ok(ele) = ele else { continue };
            let ele_file_stem = String::from(
                ele.path()
                    .file_stem()
                    .expect("could not read file stem")
                    .to_str()
                    .expect("could not convert to string"),
            );
            debug!("checking command file: {ele_file_stem}");
            if ele_file_stem == self.get_command_name() {
                file = Some(ele);
            } else if ele_file_stem.contains(self.get_command_name().as_str()) {
                config_file = Some(ele);
            }
        }
        file.map(|file| ClixFile::new_with_config(file, config_file))
    }
}

#[derive(Debug)]
pub struct ClixDirectory {
    pub(super) dir: PathBuf,
    pub(super) files: Vec<ClixFile>,
    pub(super) sub_dirs: Vec<ClixDirectory>,
}

impl ClixDirectory {
    pub fn get_command_name(&self) -> String {
        String::from(
            self.dir
                .file_name()
                .expect("could not read dir name")
                .to_str()
                .expect("could not convert dir name to str"),
        )
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
                            files.push(ClixFile::new(entry));
                        }
                    }
                }
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
    let root = read_path_buf(path);
    ClixRepo::new(root)
}

fn create_command(dir: &ClixDirectory) -> Command {
    let mut command = Command::new(dir.get_command_name());
    for file in &dir.files {
        command = command.subcommand(Command::new(file.get_command_name()));
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
            if file.get_command_name()
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
