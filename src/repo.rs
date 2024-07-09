use std::fs::*;
use std::path::*;

#[derive(Debug)]
pub struct ClixFile {
    pub(super) file: DirEntry,
}

#[derive(Debug)]
pub struct ClixDirectory {
    pub(super) dir: PathBuf,
    pub(super) files: Vec<ClixFile>,
    pub(super) sub_dirs: Vec<ClixDirectory>,
}

fn read_path_buf(path: PathBuf) -> ClixDirectory {
    println!("reading path: {path:?}");
    let mut files: Vec<ClixFile> = Vec::new();
    let mut directories: Vec<ClixDirectory> = Vec::new();

    path.read_dir()
        .expect("cannot read directory")
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        directories.push(read_path_buf(entry.path()));
                    } else {
                        files.push(ClixFile { file: entry });
                    }
                }
            }
        });

    ClixDirectory {
        dir: path,
        files,
        sub_dirs: directories,
    }
}

pub fn load_directory() -> ClixDirectory {
    const DIR: &str = "/home/locuris/code/clix/test-repo/clix-engage";
    let path = PathBuf::from(DIR);
    read_path_buf(path)
}
