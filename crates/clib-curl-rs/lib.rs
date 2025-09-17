use std::ffi::{CString, c_char};

#[unsafe(no_mangle)]
pub extern "C" fn url() {
    todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn version() -> *mut c_char {
    match CString::new(env!("CARGO_PKG_VERSION")) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
