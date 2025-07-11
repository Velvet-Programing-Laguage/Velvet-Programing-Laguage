mod ffi;
mod interpreter;
mod modules;
mod parser;
mod types;

use ffi::*;
use modules::*;

#[no_mangle]
pub extern "C" fn velvet_init(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = init(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_python_requests(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = python_requests(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_cpp_boost(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = cpp_boost(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_csharp_json(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = csharp_json(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_ruby_httparty(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = ruby_httparty(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_js_axios(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = js_axios(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_rust_flate2(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = rust_flate2(args_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn velvet_java_jython(args: *const c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or("") };
    let result = java_jython(args_str);
    CString::new(result).unwrap().into_raw()
}
