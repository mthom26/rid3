use log::{self, Level, Log, Metadata, Record};
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct LogRecord {
    // pub time: TODO,
    pub level: Level,
    pub msg: String,
}

pub struct Logger {
    pub items: Mutex<Vec<LogRecord>>,
    pub index: Mutex<usize>,
}

impl Logger {
    pub fn next(&self) {
        let len = self.items.lock().expect("Could not acquire lock").len();
        let index = *self.index.lock().expect("Could not acquire lock");
        if len > 0 && index < len - 1 {
            *self.index.lock().expect("Could not acquire lock") = index + 1;
        }
    }

    pub fn prev(&self) {
        let index = *self.index.lock().expect("Could not acquire lock");
        if index > 0 {
            *self.index.lock().expect("Could not acquire lock") = index - 1;
        }
    }
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

            let items = &mut *self.items.lock().unwrap();
            items.push(record);
            // Adjust index so new logs are displayed as they are added
            let mut index = self.index.lock().unwrap();
            let i = *index;
            if items.len() - i > 8 {
                // 8 is the current height of the log window rendered
                // *index += 1;
                *index = items.len() - 8;
            }
        }
    }

    fn flush(&self) {}
}
