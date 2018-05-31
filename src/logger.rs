use std::sync::Mutex;

lazy_static! {
    pub static ref LOGGER: Mutex<Option<LoggerSender>> = Mutex::new(None);
}

pub type LoggerSender = ::std::sync::mpsc::Sender<(String, Log)>;

pub fn set_logger(sender: LoggerSender) {
    *LOGGER.lock().unwrap() = Some(sender);
}

pub enum Log {
    Error(String),
    Warn(String),
    Info(String),
    Debug(String),
    Trace(String),
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!($crate::logger::Log::Error(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log!($crate::logger::Log::Warn(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!($crate::logger::Log::Info(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log!($crate::logger::Log::Debug(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        log!($crate::logger::Log::Trace(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        let log = ({
            use PLUGIN_NAME;
            PLUGIN_NAME.to_string()
        }, $msg);
        let logger = &$crate::logger::LOGGER;

        match (*logger.lock().as_ref().unwrap()).as_ref() {
            Some(s) => s.send(log).unwrap_or(()),
            _ => (),
        }
    }
}