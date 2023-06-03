use ansi_term::Colour;
use log::{Level, LevelFilter, Log, Metadata, Record};

struct RustOut;

impl Log for RustOut {
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

static LOGGER: RustOut = RustOut;

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
}
