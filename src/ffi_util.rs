use cassandra::cass_bool_t;
use std::slice;
use std::str;

use std::str::Utf8Error;

/// Convert C pointer-and-length to Rust `String`. Fail if it is not valid UTF-8.
///
/// # Safety
///
/// Safety concerns are identical to [`std::slice::from_raw_parts`];
/// please see those docs for details.
pub unsafe fn raw2utf8(
    data: *const ::std::os::raw::c_char,
    length: usize,
) -> Result<String, Utf8Error> {
    let slice = slice::from_raw_parts(data as *const u8, length as usize);
    Ok(str::from_utf8(slice)?.to_owned())
}

impl From<cass_bool_t> for bool {
    fn from(b: cass_bool_t) -> Self {
        match b {
            cass_bool_t::cass_true => true,
            cass_bool_t::cass_false => false,
        }
    }
}

impl From<bool> for cass_bool_t {
    fn from(b: bool) -> Self {
        if b {
            cass_bool_t::cass_true
        } else {
            cass_bool_t::cass_false
        }
    }
}
