use std::process::Command;

use log::{error, info};

use crate::model::command::ClixCommand;

use super::handle_arguments;

pub(super) fn execute_powershell_script(clix_command: ClixCommand) {
    let path = clix_command.file().file_path().path();
    let mut command = Command::new("powershell");

    let args = clix_command.command();

    command.arg(path);

    if let Some(config) = clix_command.file().get_config() {
        command = handle_arguments(args, command, config)
    }

    info!("executing powershell command: {command:?}");

    match command.output() {
        Ok(output) => {
            let out =
                String::from_utf8(output.stdout).expect("could not read command output as string");

            let lines = out.split("\\n");

            for line in lines {
                println!("{line}");
            }
        }
        Err(error) => error!("failed to execute powershell command. {error}"),
    }
}
