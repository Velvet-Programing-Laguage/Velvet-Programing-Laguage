mod config;
mod ffi;
mod interpreter;
mod logger;
mod modules;
mod parser;
mod types;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use config::Config;
use logger::Logger;

#[no_mangle]
pub extern "C" fn velvet_init(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let config = Config::load("vel.json").unwrap_or_default();
    let logger = Logger::new(config.debug);
    logger.log(&format!("Initializing Velvet with args: {}", args_str));
    let result = modules::init(args_str);
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
