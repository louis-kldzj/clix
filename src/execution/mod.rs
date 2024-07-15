use std::process::Command;

use clap::ArgMatches;
use log::{debug, error, warn};

use crate::model::{command::ClixCommand, config::CommandConfiguration};

mod system_info;
#[cfg(target_os = "linux")]
mod unix_execution;
#[cfg(target_os = "windows")]
mod windows_execution;

pub fn execute_command(command: ClixCommand) {
    execute_os_command(command)
}

#[cfg(target_os = "linux")]
fn execute_os_command(command: ClixCommand) {
    let file = command.file();

    match file.get_file_extension().as_str() {
        "sh" => unix_execution::execute_bash_script(command),
        _ => warn!("unhandled file type on linux: {file:?}"),
    }
}

#[cfg(target_os = "windows")]
fn execute_os_command(command: ClixCommand) {
    let file = command.file();

    match file.get_file_extension().as_str() {
        "ps1" => windows_execution::execute_powershell_script(command),
        _ => warn!("unhandled file type on windows: {file:?}"),
    }
}

fn handle_arguments(
    args: &ArgMatches,
    mut command: Command,
    config: CommandConfiguration,
) -> Command {
    if let Some(config_args) = config.arguments {
        for arg_entry in config_args {
            debug!("attempting to get argument from command line: {arg_entry:?}");
            if let Some(arg) = args.get_one::<String>(arg_entry.name.as_str()) {
                command.arg(arg);
            } else {
                debug!("could not read argument from command line");
                if arg_entry.required {
                    error!("required argument not provided. I'm about to panic");
                    error!("args:{args:?}");
                    error!("command:{command:?}");
                    error!("arg:{arg_entry:?}");
                    panic!("told you")
                }
            }
        }
    }
    command
}
