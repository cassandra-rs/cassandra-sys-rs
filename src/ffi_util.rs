use std::slice;
use std::str;
use cassandra::CassError;
use cassandra::cass_bool_t;
use cassandra::CassValueType_;

use std::str::Utf8Error;

pub unsafe fn raw2utf8(data: *const i8, length: usize) -> Result<String, Utf8Error> {
    let slice = slice::from_raw_parts(data as *const u8, length as usize);
    Ok(try!(str::from_utf8(slice)).to_owned())
}

impl PartialEq for CassError {
    fn eq(&self, other: &CassError) -> bool {
        self == other
    }
}

impl PartialEq for cass_bool_t {
    fn eq(&self, other: &cass_bool_t) -> bool {
        self == other
    }
}

impl PartialEq for CassValueType_ {
    fn eq(&self, other: &CassValueType_) -> bool {
        self == other
    }
}

impl Into<bool> for cass_bool_t {
    fn into(self) -> bool {
        match self {
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
