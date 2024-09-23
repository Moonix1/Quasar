use std::io::Write;

use colored::Colorize;
use log::Level;

pub struct Logger {}

impl Logger {
    pub fn init() -> Logger {
        env_logger::Builder::new()
            .format(|buf, record| {
                let level_color = match record.level() {
                    Level::Error => "red".to_string(),
                    Level::Warn => "yellow".to_string(),
                    Level::Info => "blue".to_string(),
                    Level::Debug => "blue".to_string(),
                    Level::Trace => "white".to_string(),
                };
                let msg = format!("[{}]: {}",
                    record.level().to_string(),
                    record.args().to_string()).color(level_color);
                writeln!(buf, "{}", msg)
            })
            .filter(None, log::LevelFilter::max())
            .init();
        
        Logger { }
    }
}