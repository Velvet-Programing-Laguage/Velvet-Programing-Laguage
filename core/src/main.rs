use velvet_core::{parse_velvet, Interpreter};
use std::env;
use std::fs;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_com_velvet_VelvetJNI_initJNI(env: JNIEnv, _class: JClass) {
    println!("JNI initialized for Velvet");
}

#[no_mangle]
pub extern "system" fn Java_com_velvet_VelvetJNI_runPygame(_env: JNIEnv, _class: JClass, _args: JString) -> jstring {
    _env.new_string("Pygame initialized via JNI").unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_velvet_VelvetJNI_runTk(_env: JNIEnv, _class: JClass, _args: JString) -> jstring {
    _env.new_string("Tk initialized via JNI").unwrap().into_inner()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).expect("Provide a .vel file");
    let code = fs::read_to_string(file).expect("Failed to read file");
    let ast = parse_velvet(&code).expect("Parsing failed");
    let mut interpreter = Interpreter::new();
    let gui_data = interpreter.execute(ast);
    println!("{}", gui_data);
}
