//! C ABI surface over emf-core for the emf gem's FFI binding
//! (leptris/parsanol-ffi style: opaque handles + buffer protocol;
//! UTF-8 on both faces).

/// Placeholder ABI entry point — replaced by the real surface in #15.
static RS_VERSION: &[u8] = b"0.1.0\0";

#[no_mangle]
pub extern "C" fn emf_rs_version() -> *const std::os::raw::c_char {
    RS_VERSION.as_ptr() as *const std::os::raw::c_char
}
