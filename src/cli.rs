use std::process;

pub fn print_help() {
    println!("\x1b[1;34mVelvet CLI v1.4\x1b[0m");
    println!("\x1b[1;36m  vel help\x1b[0m           - Show this help");
    println!("\x1b[1;36m  vel start [file]\x1b[0m   - Run program (default: main.velvet)");
    println!("\x1b[1;36m  vel update\x1b[0m         - Update libraries");
    println!("\x1b[1;36m  vel install <.> <manager> install <lib>\x1b[0m - Install library (e.g., vel install <.> gem install bundler)");
    println!("\x1b[1;36m  vel build\x1b[0m          - Compile to executable");
    println!("\x1b[1;36m  vel init\x1b[0m           - Init new project");
    println!("\x1b[1;36m  vel debug [file]\x1b[0m   - Run with debug output");
    println!("\x1b[1;36m  vel test\x1b[0m           - Run tests");
    println!("\x1b[1;36m  vel clean\x1b[0m          - Clean build artifacts");
    println!("\x1b[1;36m  vel version\x1b[0m        - Show version");
    println!("\x1b[1;36m  vel repl\x1b[0m           - Start REPL");
    println!("\x1b[1;36m  vel fmt\x1b[0m            - Format files");
    println!("\x1b[1;36m  vel list-libs\x1b[0m      - List libraries");
    println!("\x1b[1;36m  vel check\x1b[0m          - Check project config");
}

pub fn success(message: &str) {
    println!("\x1b[1;32mOK:\x1b[0m {}", message);
}

pub fn error(message: &str) {
    eprintln!("\x1b[1;31mERR:\x1b[0m {}", message);
}

pub fn info(message: &str) {
    println!("\x1b[1;33mINFO:\x1b[0m {}", message);
}

pub fn debug(message: &str) {
    println!("\x1b[1;35mDBG:\x1b[0m {}", message);
}

pub fn version() -> &'static str {
    "1.4"
}

pub fn print_version() {
    println!("\x1b[1;34mVelvet v{}\x1b[0m", version());
}
