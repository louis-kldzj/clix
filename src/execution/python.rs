use std::process::Command;

use log::info;

use crate::{execution::run_command_and_print_output, model::command::ClixCommand};

use super::handle_arguments;

pub fn execute_python_command(clix_command: ClixCommand) {
    let path = clix_command.file().file_path().path();
    let mut command = Command::new("python");

    let args = clix_command.command();

    command.arg(path);

    if let Some(config) = clix_command.file().try_get_config() {
        command = handle_arguments(args, command, config);
    }

    info!("executing python command: {command:?}");

    run_command_and_print_output(command);
}
