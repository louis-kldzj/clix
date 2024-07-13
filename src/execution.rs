use std::path::PathBuf;
use std::process::Command;

use log::{error, info, warn};

use crate::model::command::ClixCommand;

pub fn execute_command(command: ClixCommand) {
    let file = command.file();
    if let Some(extension) = file.file_path().path().extension() {
        match extension.to_str().expect("could not convert OSstr to str") {
            "sh" => execute_bash_script(command),
            _ => warn!("unhandled file type: {file:?}"),
        }
    }
}

fn execute_bash_script(file: ClixCommand) {
    let file = file.file().file_path().path();
    let mut command = Command::new("bash");

    command.arg(file);

    match command.output() {
        Ok(output) => {
            info!("that's how you run a command! {output:?}");

            let mut out =
                String::from_utf8(output.stdout).expect("could not read command output as string");

            out.split("\\n");

            println!("{out:?}");
        }
        Err(error) => error!("that's not how you run a command. {error:?}"),
    }
}
