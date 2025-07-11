mod config;
mod error;
mod ffi;
mod interpreter;
mod logger;
mod module_registry;
mod modules;
mod parser;
mod plugin_system;
mod repl;
mod runtime;
mod types;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use config::Config;
use logger::Logger;
use module_registry::ModuleRegistry;
use runtime::Runtime;

/// Helper: safely convert C string pointer to Rust &str
fn cstr_to_str(ptr: *const c_char) -> Result<&str, String> {
    if ptr.is_null() {
        return Err("Error: null pointer received".to_string());
    }
    unsafe { 
        CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| "Error: invalid UTF-8".to_string())
    }
}

/// Helper: safely create CString from Rust String
fn result_to_cstring(result: String) -> *mut c_char {
    match CString::new(result) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => CString::new("Error: result contains null byte").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn velvet_init(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let config = Config::load("vel.json").unwrap_or_default();
    let logger = Logger::new(config.debug);
    logger.info(&format!("Initializing Velvet with args: {}", args_str));

    let mut registry = ModuleRegistry::new();
    registry.register_builtin_modules();
    let runtime = Runtime::new(config.clone());
    let result = runtime.init(args_str);
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_python_requests(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::python::requests(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_cpp_boost(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::cpp::boost(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_csharp_json(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::csharp::json(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_ruby_httparty(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::ruby::httparty(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_js_axios(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::javascript::axios(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_rust_flate2(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::rust::flate2(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_java_jython(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::java::jython(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_tauri_gui(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::tauri::gui(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_wayland_gui(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::wayland::gui(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_ai_tensorflow(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::ai::tensorflow(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_ai_pytorch(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::ai::pytorch(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_perf_parallel(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::perf::parallel(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_perf_crypto(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::perf::crypto(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_db_sqlite(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::db::sqlite(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_net_websocket(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::net::websocket(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_gpu_cuda(args: *const c_char) -> *mut c_char {
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let result = match modules::gpu::cuda(args_str) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    };
    result_to_cstring(result)
}

#[no_mangle]
pub extern "C" fn velvet_async_exec(module: *const c_char, args: *const c_char) -> *mut c_char {
    let module_str = match cstr_to_str(module) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let args_str = match cstr_to_str(args) {
        Ok(s) => s,
        Err(e) => return result_to_cstring(e),
    };
    let rt = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(e) => return result_to_cstring(format!("Error: failed to create Tokio runtime: {}", e)),
    };
    let result = rt.block_on(async {
        ModuleRegistry::execute_async(module_str, args_str).await
    });
    match result {
        Ok(res) => result_to_cstring(res),
        Err(e) => result_to_cstring(format!("Error: {}", e)),
    }
}
