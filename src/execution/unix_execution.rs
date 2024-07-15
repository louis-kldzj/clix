use std::process::Command;

use log::{debug, error, info, warn};

use crate::model::command::ClixCommand;

use super::{handle_arguments, run_command_and_print_output};

pub(super) fn execute_bash_script(clix_command: ClixCommand) {
    let path = clix_command.file().file_path().path();
    let mut command = Command::new("bash");

    let args = clix_command.command();

    command.arg(path);

    if let Some(config) = clix_command.file().get_config() {
        command = handle_arguments(args, command, config);
    }

    info!("executing bash command: {command:?}");

    run_command_and_print_output(command);
}
