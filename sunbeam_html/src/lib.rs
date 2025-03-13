mod document;
mod html_elements;
mod lang_system;
mod parser;

struct HtmlMetadata {}

// fn new_document() -> Document {}

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
