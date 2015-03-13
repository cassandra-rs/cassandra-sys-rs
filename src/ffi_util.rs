use cassandra::*;

use std::mem;
use std::slice;
use std::str;

pub fn cassvalue2cassstring<'a>(value:&CassValue) -> Result<CassString,CassError> {unsafe{
    let mut cass_string:CassString = mem::uninitialized();
    cass_value_get_string(value, &mut cass_string);
    Ok(cass_string)
}}

pub fn str2cass_string(query:&str) -> CassString {unsafe{
    cass_string_init_n(query.as_ptr() as *const i8,query.len() as u64)
}}

pub fn str2ref(query:&str) -> *const i8 {
    query.as_ptr() as *const i8
}

pub unsafe fn raw2utf8(data:*const i8, length:u64) -> String {
    let slice = slice::from_raw_parts(data as *const u8,length as usize);
    str::from_utf8(slice).unwrap().to_string()
}
