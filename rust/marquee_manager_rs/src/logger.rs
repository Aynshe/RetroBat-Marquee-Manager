use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

pub fn init() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("marquee_manager_rs.log").unwrap(),
    )
    .expect("Failed to initialize logger");
}
