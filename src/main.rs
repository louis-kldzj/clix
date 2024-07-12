use log::{info, warn, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

mod model;

fn main() {
    configure_logging();
    let repo = model::load_directory();
    let file = repo.clap_file();
    match file {
        Some(clix_file) => info!("we have a file! {clix_file:?}"),
        None => warn!("we don't have a file!"),
    }
}

mod test {
    use super::*;

    fn main() {
        let repo = model::load_directory();
        println!("wow! {repo:?}");
        println!();

        let command = repo.clap();
        println!("clap! {command}");

        let matches = command.get_matches();
        println!("match! {matches:?}");
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
