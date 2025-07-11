pub struct Logger {
    debug: bool,
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        Logger { debug }
    }

    pub fn log(&self, message: &str) {
        if self.debug {
            println!("[DEBUG] {}", message);
        } else {
            println!("{}", message);
        }
    }
}
