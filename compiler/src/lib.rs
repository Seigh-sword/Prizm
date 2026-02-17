// Prizm Compiler Library
// Exports compiler functionality for use as a DLL/shared library

pub mod attributes;
pub mod lexer;
pub mod stdlib;

use lexer::Lexer;

/// Compile a Prizm source file
/// Returns compiled binary data
#[no_mangle]
pub extern "C" fn compile_prizm(source_code: *const u8, source_len: usize) -> *const u8 {
    if source_code.is_null() {
        return std::ptr::null();
    }

    let source = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(source_code, source_len))
            .unwrap_or("")
    };

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Convert tokens to binary format
    let binary_data = format!("{:?}", tokens);
    let bytes = binary_data.into_bytes();
    let boxed_bytes = bytes.into_boxed_slice();
    Box::into_raw(boxed_bytes) as *const u8
}

/// Execute compiled Prizm code
#[no_mangle]
pub extern "C" fn execute_prizm(binary_data: *const u8, binary_len: usize) -> i32 {
    if binary_data.is_null() {
        return -1;
    }

    // Placeholder for execution logic
    0
}

/// Get compiler version
#[no_mangle]
pub extern "C" fn get_version() -> *const u8 {
    let version = "0.1.0\0";
    version.as_ptr() as *const u8
}

/// Free allocated memory
#[no_mangle]
pub extern "C" fn free_memory(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}