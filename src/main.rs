use execution::execute_command;
use log::{info, warn, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use model::command::clap_file;

mod config;
mod execution;
mod model;

fn main() {
    configure_logging();
    let repo = model::repo::load_directory();
    let file = clap_file(&repo);
    match file {
        Some(clix_file) => {
            info!("we have a file! {clix_file:?}");
            execute_command(clix_file.file_path().path().clone());
        }
        None => warn!("we don't have a file!"),
    }
}

fn configure_logging() {
    let log_file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} {h({l})} {M} - {m}{n}",
        )))
        .build("debug.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logger", Box::new(log_file_appender)))
        .build(Root::builder().appender("logger").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
