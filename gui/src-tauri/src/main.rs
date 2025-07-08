use tauri::Manager;
use velvet_core::{parse_velvet, Interpreter};

#[tauri::command]
async fn get_gui_data(file: String) -> serde_json::Value {
    let code = std::fs::read_to_string(file).expect("Failed to read file");
    let ast = parse_velvet(&code).expect("Parsing failed");
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast)
}

#[tauri::command]
async fn execute_action(action: String) {
    println!("Action executed: {}", action);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_gui_data, execute_action])
        .run(tauri::generate_context!())
        .expect("Error running Tauri app");
}
