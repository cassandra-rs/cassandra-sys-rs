#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![recursion_limit = "1024"]

pub use ffi_util::*;

pub use cassandra::cass_bool_t::{cass_false, cass_true};

pub use cassandra::CassBatchType_::*;
pub use cassandra::CassCollectionType_::{
    CASS_COLLECTION_TYPE_LIST, CASS_COLLECTION_TYPE_MAP, CASS_COLLECTION_TYPE_SET,
};
pub use cassandra::CassError_::*;
pub use cassandra::CassLogLevel_::CASS_LOG_INFO;
pub use cassandra::CassSslVerifyFlags::*;
pub use cassandra::CassValueType_::*;

pub use cassandra::*;

mod cassandra;
pub mod ffi_util;
