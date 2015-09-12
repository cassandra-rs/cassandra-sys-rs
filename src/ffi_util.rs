use std::slice;
use std::str;
use std::ffi::CString;

#[inline]
pub fn str2ref(query:&str) -> *const i8 {
    CString::new(query).unwrap().as_ptr() as *const i8
}

pub unsafe fn raw2utf8(data:*const i8, length:u64) -> String {
    let slice = slice::from_raw_parts(data as *const u8,length as usize);
    str::from_utf8(slice).unwrap().to_string()
}
