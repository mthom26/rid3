use log::{self, Level, Log, Metadata, Record};
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct LogRecord {
    // pub time: TODO,
    pub level: Level,
    pub msg: String,
}

pub struct Logger {
    // TODO - Enable scrolling through list
    pub items: Mutex<Vec<LogRecord>>,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let record = LogRecord {
                level: record.level(),
                msg: record.args().to_string(),
            };
            self.items
                .lock()
                .expect("Could not acquire lock")
                .push(record);
        }
    }

    fn flush(&self) {}
}
