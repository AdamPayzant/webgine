pub mod display_data;
pub mod document;
pub mod html_elements;
pub mod parser;

pub use document::document::Document;

mod lang_system;

#[cfg(test)]
mod tests {
    use log::Level;

    struct DebugLogger;
    impl log::Log for DebugLogger {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &log::Record) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }

    static LOGGER: DebugLogger = DebugLogger;

    #[test]
    fn init() {
        _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace));
    }

    use super::*;
}
