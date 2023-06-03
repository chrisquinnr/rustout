use log::{Level, LevelFilter, Metadata, Record};

pub struct RustOut;

impl log::Log for RustOut {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] - {} - {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

/// Initializes the Rustout logger with the desired log level filter.
pub fn init_logger() {
    log::set_logger(&RustOut).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
