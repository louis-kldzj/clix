use std::process::Command;

use clap::ArgMatches;
use log::{debug, error, info};
use platform::LinuxFileTypes;

use crate::model::{command::ClixCommand, config::CommandConfiguration};

mod system_info;
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

trait FileTypeSpecifier {
    fn from_extension(extension: &str) -> CommandFileTypes;
}

enum CommandFileTypes {
    #[cfg(target_os = "linux")]
    Linux(platform::LinuxFileTypes),
    #[cfg(target_os = "windows")]
    Windows(platform::WindowsFileTypes),
    Python,
    Unhandled(String),
}

impl FileTypeSpecifier for CommandFileTypes {
    fn from_extension(extension: &str) -> Self {
        match extension {
            "py" => Self::Python,
            unhandled => {
                #[cfg(target_os = "linux")]
                return LinuxFileTypes::from_extension(unhandled);
                #[cfg(target_os = "windows")]
                return WindowsFileTypes::from_extension(unhandled);
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {

    use log::warn;

    use crate::model::command::ClixCommand;

    use super::{unix_execution, CommandFileTypes, FileTypeSpecifier};

    pub fn execute_os_command(command: ClixCommand) {
        let file = command.file();

        match file.get_file_extension().as_str() {
            "sh" => unix_execution::execute_bash_script(command),
            _ => warn!("unhandled file type on linux: {file:?}"),
        }
    }

    pub enum LinuxFileTypes {
        Bash,
    }

    impl FileTypeSpecifier for LinuxFileTypes {
        fn from_extension(extension: &str) -> CommandFileTypes {
            match extension {
                "sh" => CommandFileTypes::Linux(Self::Bash),
                unhandled => CommandFileTypes::Unhandled(unhandled.to_string()),
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {

    use log::warn;

    use crate::model::command::ClixCommand;

    use super::windows_execution;

    pub fn execute_os_command(command: ClixCommand) {
        let file = command.file();

        match file.get_file_extension().as_str() {
            "ps1" => windows_execution::execute_powershell_script(command),
            _ => warn!("unhandled file type on windows: {file:?}"),
        }
    }

    pub enum WindowsFileTypes {
        Powershell,
    }
}
