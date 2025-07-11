mod builder;
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
use builder::{build_project, package_project};
use config::Config;
use logger::Logger;
use module_registry::ModuleRegistry;
use runtime::Runtime;

#[no_mangle]
pub extern "C" fn velvet_init(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let config = Config::load("vel.json").unwrap_or_default();
    let logger = Logger::new(config.debug);
    logger.info(&format!("Initializing Velvet with args: {}", args_str));

    let mut registry = ModuleRegistry::new();
    registry.register_builtin_modules();
    let runtime = Runtime::new(config.clone());
    let result = runtime.init(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_build(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let parts: Vec<&str> = args_str.split(',').collect();
    let release = parts.get(0).map(|s| s == "release").unwrap_or(false);
    let package_type = parts.get(1).unwrap_or(&"exe");
    let result = build_project(release, package_type);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_package(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = package_project(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_python_requests(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::python::requests(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_cpp_boost(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::cpp::boost(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_csharp_json(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::csharp::json(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_ruby_httparty(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::ruby::httparty(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_js_axios(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::javascript::axios(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_rust_flate2(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::rust::flate2(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_java_jython(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::java::jython(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_tauri_gui(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::tauri::gui(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_wayland_gui(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::wayland::gui(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_ai_tensorflow(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::ai::tensorflow(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_ai_pytorch(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::ai::pytorch(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_perf_parallel(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::perf::parallel(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_perf_crypto(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::perf::crypto(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_db_sqlite(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::db::sqlite(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_net_websocket(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::net::websocket(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_gpu_cuda(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = modules::gpu::cuda(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_async_exec(module: *const c_char, args: *const c_char) -> *mut c_char {
    let module_str = unsafe { CStr::from_ptr(module).to_str().unwrap_or("") };
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        ModuleRegistry::execute_async(module_str, args_str).await
    });
    CString::new(result.unwrap_or_else(|e| format!("Error: {}", e))).unwrap().into_raw()
}
