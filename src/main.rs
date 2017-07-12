use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn main() {
    // Here we could use println! to output some message to the browser console
    // Useful for checking that the wasm binary is actually loading properly.
}

// Since web assembly only has builtin support for i32, i64, f32, and f64 types,
// Working with strings, arrays, or other data structures requires accessing them
// From a pointer to their location in memory.
fn my_string_safe(i: *mut c_char) -> String {
    unsafe {
        CStr::from_ptr(i).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub fn fix_story(i: *mut c_char) -> *mut c_char {

    let data = my_string_safe(i);
    let f = data.replace("one", "once");

    CString::new(f.as_str())
        .unwrap()
        .into_raw()

}

#[no_mangle]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
fn square(x: i32) -> i32 {
    x * x
}