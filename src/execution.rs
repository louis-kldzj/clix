use std::fs::DirEntry;
use std::process::Command;

use log::{error, info, warn};

pub fn execute_command(file: DirEntry) {
    if let Some(extension) = file.path().extension() {
        match extension.to_str().expect("could not convert OSstr to str") {
            "sh" => execute_bash_script(file),
            _ => warn!("unhandled file type: {file:?}"),
        }
    }
}

fn execute_bash_script(file: DirEntry) {
    let mut command = Command::new("bash");

    command.arg(file.path());

    match command.output() {
        Ok(output) => {
            info!("that's how you run a command! {output:?}");
            let out =
                String::from_utf8(output.stdout).expect("could not read command output as string");
            println!("{out:?}");
        }
        Err(error) => error!("that's not how you run a command. {error:?}"),
    }
}
