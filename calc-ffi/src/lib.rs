use calc::prelude::*;
use std::os::raw::c_char;
use std::ffi::CStr;

// What I want to expose:
// * single function
// * takes a C string of "input"
// * returns (if successful) the output

// parse_and_eval
//
// Arguments?
//    * const char
// Return type?
//    * integer

// returns 0 if success, returns nonzero if failure
// on success, output is updated with the result?
//
// int parse_and_eval(char*, int64_t* output);

pub use calc::Expr;

#[no_mangle]
pub extern "C" fn c_parse(
    maybe_cstr: *const c_char,
    output: *mut Expr,
) -> isize {
    // Check if cstr is valid
    if maybe_cstr.is_null() {
        return -1;
    }
    let cstr = unsafe {
        CStr::from_ptr(maybe_cstr)
    };
    let string_data = match cstr.to_str() {
        Ok(s) => s,
        Err(_e) => return -2,
    };

    // check if output is non-null
    if output.is_null() {
        return -1;
    }

    // Do parse, do eval
    let parsed = match parse(string_data) {
        Ok(p) => p,
        Err(_e) => return -3,
    };

    unsafe {
        *output = parsed;
    }

    0
}

#[no_mangle]
pub extern "C" fn c_eval(
    expr: *const Expr,
    output: *mut i64
) -> isize {
    // Check if cstr is valid
    if expr.is_null() {
        return -1;
    }

    // check if output is non-null
    if output.is_null() {
        return -1;
    }

    // if successful, set error code, return
    let evaled = match eval(unsafe {&*expr}) {
        Ok(ev) => ev,
        Err(_e) => return -4,
    };

    unsafe {
        *output = evaled;
    }

    0
}

#[no_mangle]
pub extern "C" fn parse_and_eval(
    maybe_cstr: *const c_char,
    output: *mut i64
) -> isize {
    // Check if cstr is valid
    if maybe_cstr.is_null() {
        return -1;
    }
    let cstr = unsafe {
        CStr::from_ptr(maybe_cstr)
    };
    let string_data = match cstr.to_str() {
        Ok(s) => s,
        Err(_e) => return -2,
    };

    // check if output is non-null
    if output.is_null() {
        return -1;
    }

    // Do parse, do eval
    let parsed = match parse(string_data) {
        Ok(p) => p,
        Err(_e) => return -3,
    };

    // if successful, set error code, return
    let evaled = match eval(&parsed) {
        Ok(ev) => ev,
        Err(_e) => return -4,
    };

    unsafe {
        *output = evaled;
    }

    0
}

//
// *const -> const *
// *mut   -> *
