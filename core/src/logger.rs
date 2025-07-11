use log::{debug, error, info};

pub struct Logger {
    debug: bool,
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        env_logger::init();
        Logger { debug }
    }

    pub fn info(&self, message: &str) {
        info!("{}", message);
    }

    pub fn debug(&self, message: &str) {
        if self.debug {
            debug!("{}", message);
        }
    }

    pub fn error(&self, message: &str) {
        error!("{}", message);
    }
}
