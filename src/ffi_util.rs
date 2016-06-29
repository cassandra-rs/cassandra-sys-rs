use std::slice;
use std::str;

use std::str::Utf8Error;

pub unsafe fn raw2utf8(data: *const i8, length: usize) -> Result<String, Utf8Error> {
    let slice = slice::from_raw_parts(data as *const u8, length as usize);
    Ok(try!(str::from_utf8(slice)).to_owned())
}
