use std::process::Command;

use clap::ArgMatches;
use log::{debug, error, info};

use crate::model::{command::ClixCommand, config::CommandConfiguration};

mod unix_execution;
mod windows_execution;

pub fn execute_command(command: ClixCommand) {
    platform::execute_os_command(command)
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

fn run_command_and_print_output(mut command: Command) {
    match command.output() {
        Ok(output) => {
            let out =
                String::from_utf8(output.stdout).expect("could not read command output as string");

            info!("output from command execution: {out:?}");

            let lines = out.split("\\n");

            for line in lines {
                println!("{line}");
            }
        }
        Err(error) => {
            error!("failed to execute command. {error}");
            println!("{error}");
        }
    }
}

pub trait FileTypeSpecifier {
    fn from_extension(extension: &str) -> CommandFileType;
}

#[derive(Clone, Debug)]
pub enum CommandFileType {
    #[cfg(target_os = "linux")]
    Linux(platform::PlatformSpecificFileType),
    #[cfg(target_os = "windows")]
    Windows(platform::PlatformSpecificFileType),
    Python,
    Unhandled(String),
}

impl FileTypeSpecifier for CommandFileType {
    fn from_extension(extension: &str) -> Self {
        match extension {
            "py" => Self::Python,
            unhandled => platform::PlatformSpecificFileType::from_extension(unhandled),
        }
    }
}

//NOTE: technically we can run bash on windows with WSL and it appears it handles it fine without
//extra logic, however, I will not support bash on windows right now as it acts on a sepereate file
//system

#[cfg(target_os = "linux")]
mod platform {

    use log::warn;

    use crate::model::command::ClixCommand;

    use super::{unix_execution, CommandFileType, FileTypeSpecifier};

    pub fn execute_os_command(command: ClixCommand) {
        let file = command.file();

        match file.get_file_extension().as_str() {
            "sh" => unix_execution::execute_bash_script(command),
            _ => warn!("unhandled file type on linux: {file:?}"),
        }
    }

    #[derive(Clone, Debug)]
    pub enum PlatformSpecificFileType {
        Bash,
    }

    impl FileTypeSpecifier for PlatformSpecificFileType {
        fn from_extension(extension: &str) -> CommandFileType {
            match extension {
                "sh" => CommandFileType::Linux(Self::Bash),
                unhandled => CommandFileType::Unhandled(unhandled.to_string()),
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) mod platform {

    use log::warn;

    use crate::model::command::ClixCommand;

    use super::{windows_execution, CommandFileType, FileTypeSpecifier};

    pub fn execute_os_command(command: ClixCommand) {
        let file = command.file();

        match file.get_file_extension().as_str() {
            "ps1" => windows_execution::execute_powershell_script(command),
            _ => warn!("unhandled file type on windows: {file:?}"),
        }
    }

    #[derive(Clone, Debug)]
    pub enum PlatformSpecificFileType {
        Powershell,
    }

    impl FileTypeSpecifier for PlatformSpecificFileType {
        fn from_extension(extension: &str) -> super::CommandFileType {
            match extension {
                "ps1" => CommandFileType::Windows(Self::Powershell),
                unhandled => CommandFileType::Unhandled(unhandled.to_string()),
            }
        }
    }
}
