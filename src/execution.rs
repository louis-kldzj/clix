use core::panic;
use std::process::Command;

use log::{debug, error, info, warn};

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

fn execute_bash_script(clix_command: ClixCommand) {
    let path = clix_command.file().file_path().path();
    let mut command = Command::new("bash");

    let args = clix_command.command();

    command.arg(path);

    if let Some(config) = clix_command.file().get_config() {
        if let Some(arguments) = config.arguments {
            for arg_entry in arguments {
                debug!("attempting to get argument from command line: {arg_entry:?}");
                if args.contains_id(arg_entry.name.as_str()) {
                    warn!("should have it!");
                }
                if let Some(arg) = args.get_one::<String>(arg_entry.name.as_str()) {
                    command.arg(arg);
                } else {
                    debug!("could not retrieve arguments.");
                    if arg_entry.required {
                        error!("required argument not provided. panicking!");
                        error!("command: {clix_command:?}");
                        panic!("argument {:?} not provided", arg_entry.name)
                    }
                }
            }
        }
    }

    info!("executing command: {command:?}");

    match command.output() {
        Ok(output) => {
            info!("that's how you run a command! {output:?}");

            let out =
                String::from_utf8(output.stdout).expect("could not read command output as string");

            let lines = out.split("\\n");

            for line in lines {
                println!("{line}");
            }
        }
        Err(error) => error!("that's not how you run a command. {error:?}"),
    }
}
