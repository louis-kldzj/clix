#[cfg(target_os = "linux")]
const SUPPORTED_FILE_TYPES: [&str; 1] = ["sh"];

#[cfg(target_os = "windows")]
const SUPPORTED_FILE_TYPES: [&str; 1] = ["ps1"];

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use std::{path::PathBuf, process::Command};

    #[test]
    fn can_run_powershell() {
        print!("trying to print");
        let cd = std::env::current_dir().expect("could not get current directory on windows");
        let path = PathBuf::from(
            format!("{cd:?}\\test-repo\\win\\test.ps1")
                .replace('/', "")
                .replace('\"', ""),
        );
        if !path.exists() {
            panic!("cannase get path on windows :( - {path:?}")
        }

        println!("{:?}", Command::new("powershell").arg(path).output());
    }
}
