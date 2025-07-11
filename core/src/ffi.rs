use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn init(args: &str) -> String {
    format!("Velvet core initialized with args: {}", args)
}
