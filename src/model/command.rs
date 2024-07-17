use clap::{Arg, ArgMatches, Command};
use log::{debug, info};

use super::repo::{dir::ClixDirectory, file::ClixFile, ClixRepo};

pub fn clap_file_from_stdin(repo: &ClixRepo) -> Option<ClixCommand> {
    debug!("clapping file");
    let matches = clap(repo).get_matches();
    walk_repo(&matches, repo.root_dir())
}

#[cfg(test)]
pub fn clap_file_from_str(repo: &ClixRepo, input: Vec<&str>) -> Option<ClixCommand> {
    if let Ok(matches) = clap(repo).try_get_matches_from(input) {
        walk_repo(&matches, repo.root_dir())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ClixCommand {
    file: ClixFile,
    command: ArgMatches,
}

impl ClixCommand {
    pub fn file(&self) -> &ClixFile {
        &self.file
    }

    pub fn command(&self) -> &ArgMatches {
        &self.command
    }
}

fn walk_repo(arg_match: &ArgMatches, clix_dir: &ClixDirectory) -> Option<ClixCommand> {
    let command_name = arg_match.subcommand_name().unwrap();
    let dir_name = clix_dir.get_command_name();
    let mut nxt_match = arg_match;
    debug!("walking repo for {command_name} in {dir_name}");
    if let Some((cmd_name, next_match)) = arg_match.subcommand() {
        debug!("got subcommand {cmd_name}");
        for dir in clix_dir.sub_dirs() {
            if dir.get_command_name() == cmd_name {
                return walk_repo(next_match, dir);
            }
        }
        nxt_match = next_match;
    }

    debug!("should be on last command...");
    for file in clix_dir.files() {
        debug!("checking file: {file:?}");
        if file.get_file_name()
            == arg_match
                .subcommand_name()
                .expect("could not get subcommand name")
        {
            return Some(ClixCommand {
                file: file.clone(),
                command: nxt_match.clone(),
            });
        }
    }

    None
}

fn clap(dir: &ClixRepo) -> Command {
    create_command(dir.root_dir())
}

fn create_command(dir: &ClixDirectory) -> Command {
    info!("creating command");
    let mut command = Command::new(dir.get_command_name());
    for file in dir.files() {
        if file.get_file_name().starts_with('.') {
            continue;
        }
        let mut subcommand = Command::new(file.get_file_name());
        if let Some(config) = file.get_config() {
            info!("config file found");
            if let Some(arguments) = config.arguments {
                info!("argument config present");
                for arg in arguments {
                    info!("adding argument: {arg:?}");
                    subcommand = subcommand.arg(Arg::new(arg.name).required(arg.required));
                }
            }
        }
        command = command.subcommand(subcommand);
    }
    for sub_dir in dir.sub_dirs() {
        command = command.subcommand(create_command(sub_dir));
    }
    command
}
