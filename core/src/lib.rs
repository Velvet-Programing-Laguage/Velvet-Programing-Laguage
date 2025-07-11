pub mod parser;
pub mod interpreter;
pub mod stdlib_1;
pub mod stdlib_2;

pub use parser::parse_velvet;
pub use interpreter::{Interpreter, RuntimeValue};

#[no_mangle]
pub extern "C" fn velvet_python_requests(url: *const c_char, method: *const c_char) -> *const c_char {
    // Implementation in stdlib_1.rs
}

#[no_mangle]
pub extern "C" fn velvet_cpp_boost(action: *const c_char, input: *const c_char) -> *const c_char {
    // Implementation in stdlib_1.rs
}

#[no_mangle]
pub extern "C" fn velvet_csharp_json(action: *const c_char, input: *const c_char) -> *const c_char {
    // Implementation in stdlib_2.rs
}

#[no_mangle]
pub extern "C" fn velvet_ruby_httparty(url: *const c_char) -> *const c_char {
    // Implementation in stdlib_2.rs
}

#[no_mangle]
pub extern "C" fn velvet_js_axios(url: *const c_char) -> *const c_char {
    // Implementation in stdlib_2.rs
}

#[no_mangle]
pub extern "C" fn velvet_rust_flate2(action: *const c_char, data: *const c_char) -> *const c_char {
    // Implementation in stdlib_1.rs
}

#[no_mangle]
pub extern "C" fn velvet_java_jython(script: *const c_char) -> *const c_char {
    // Implementation in stdlib_2.rs
}
