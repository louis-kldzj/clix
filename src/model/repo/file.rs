use crate::{
    execution::CommandFileType,
    model::config::{get_command_configuration, CommandConfiguration},
};

use super::ClixPath;

#[derive(Debug, Clone)]
pub enum FileType {
    Command(CommandFileType),
    Config,
}

#[derive(Debug, Clone)]
pub struct ClixFile {
    file: ClixPath,
    config_file: Option<ClixPath>,
    file_type: CommandFileType,
}

impl ClixFile {
    pub fn new(file: ClixPath) -> Self {
        let file_type = file
            .file_type()
            .expect("by this point we should have a file type");
        let mut clix = ClixFile {
            file,
            config_file: None,
            file_type,
        };
        clix.try_set_config_file();
        clix
    }

    pub fn file_path(&self) -> &ClixPath {
        &self.file
    }

    pub fn file_name(&self) -> String {
        self.file.name()
    }

    pub fn file_type(&self) -> &CommandFileType {
        &self.file_type
    }

    pub fn try_get_config(&self) -> Option<CommandConfiguration> {
        if let Some(config_file) = &self.config_file {
            if let Ok(config) = get_command_configuration(config_file.path().clone()) {
                return Some(config);
            }
        }
        None
    }

    fn try_set_config_file(&mut self) {
        let file_name = self.file.name();
        for neighbour in self.file.get_neighbours_or_contents() {
            if !neighbour.is_dir() {
                let path = ClixPath::new(neighbour);
                let name = path.name();
                if name != file_name && name.contains(file_name.as_str()) {
                    self.config_file = Some(path);
                    return;
                }
            }
        }
    }
}
