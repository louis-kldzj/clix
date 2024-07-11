use std::fs::*;
use std::path::*;

use clap::Command;

pub enum ClixObject {
    CommandDirectory,
    CommandFile,
}

#[derive(Debug)]
pub struct ClixFile {
    pub(super) file: DirEntry,
}

impl ClixFile {
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
    println!("reading path: {path:?}");
    let mut files: Vec<ClixFile> = Vec::new();
    let mut directories: Vec<ClixDirectory> = Vec::new();

    path.read_dir()
        .expect("cannot read directory")
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if !get_last_path_component_as_string(entry.path()).starts_with('.') {
                            directories.push(read_path_buf(entry.path()));
                        }
                    } else {
                        files.push(ClixFile { file: entry });
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
}

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
