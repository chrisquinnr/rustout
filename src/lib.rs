use ansi_term::Colour;
use log::{Level, LevelFilter, Metadata, Record};

pub struct RustOut;

impl log::Log for RustOut {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = match record.level() {
                Level::Error => Colour::Red.bold().paint("ERROR"),
                Level::Warn => Colour::Yellow.bold().paint("WARN"),
                Level::Info => Colour::Green.bold().paint("INFO"),
                Level::Debug => Colour::Blue.bold().paint("DEBUG"),
                Level::Trace => Colour::Purple.bold().paint("TRACE"),
            };

            let message = format!("{} → {}", level_str, record.args());

            println!("{}", message);
        }
    }

    fn flush(&self) {}
}

/// Initializes the Rustout logger with the desired log level filter.
pub fn init_logger() {
    log::set_logger(&RustOut).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
