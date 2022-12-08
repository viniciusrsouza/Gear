use log::{info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn init() {
    let encoder = PatternEncoder::new("{h([{d(%Y-%m-%d %H:%M:%S)}][{t}] {l} - {m}{n})}");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("core", LevelFilter::Trace))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
    info!(target: "GEAR", "Log initialized.");
}
