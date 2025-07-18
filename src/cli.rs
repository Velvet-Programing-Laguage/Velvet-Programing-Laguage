use std::process;

pub fn print_help() {
    println!("\x1b[1;34mVelvet CLI Commands (v1.3)\x1b[0m");
    println!("\x1b[1;36m  vel help           \x1b[0m- Show this help message");
    println!("\x1b[1;36m  vel start [file]   \x1b[0m- Run the Velvet program (default: main.velvet)");
    println!("\x1b[1;36m  vel update         \x1b[0m- Update external libraries in .velvet_library");
    println!("\x1b[1;36m  vel install <lang> <command> \x1b[0m- Install library (e.g., vel install pip install tk)");
    println!("\x1b[1;36m  vel build          \x1b[0m- Compile the program into a single executable");
    println!("\x1b[1;36m  vel init           \x1b[0m- Initialize a new Velvet project");
    println!("\x1b[1;36m  vel debug [file]   \x1b[0m- Run with debugging output (default: main.velvet)");
    println!("\x1b[1;36m  vel test           \x1b[0m- Run project tests");
    println!("\x1b[1;36m  vel clean          \x1b[0m- Clean build artifacts");
    println!("\x1b[1;36m  vel version        \x1b[0m- Show Velvet version");
    println!("\x1b[1;36m  vel repl           \x1b[0m- Start interactive REPL");
    println!("\x1b[1;36m  vel fmt            \x1b[0m- Format project files");
    println!("\x1b[1;36m  vel list-libs      \x1b[0m- List installed libraries");
    println!("\x1b[1;36m  vel check          \x1b[0m- Check project configuration");
}

pub fn success(message: &str) {
    println!("\x1b[1;32mSuccess:\x1b[0m {}", message);
}

pub fn error(message: &str) {
    eprintln!("\x1b[1;31mError:\x1b[0m {}", message);
}

pub fn info(message: &str) {
    println!("\x1b[1;33mInfo:\x1b[0m {}", message);
}

pub fn debug(message: &str) {
    println!("\x1b[1;35mDebug:\x1b[0m {}", message);
}

pub fn version() -> &'static str {
    "1.3"
}

pub fn print_version() {
    println!("\x1b[1;34mVelvet Programming Language v{}\x1b[0m", version());
}
