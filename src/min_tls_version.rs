//! Custom module exposing the not-yet-released minimum TLS version features in the datastax
//! driver.

use crate::cassandra::{CassError, CassSsl};

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CassSslTlsVersion_ {
    CASS_SSL_VERSION_TLS1 = 0,
    CASS_SSL_VERSION_TLS1_1 = 1,
    CASS_SSL_VERSION_TLS1_2 = 2,
}
pub use self::CassSslTlsVersion_ as CassSslTlsVersion;

extern "C" {
    pub fn cass_ssl_set_min_protocol_version(
        ssl: *mut CassSsl,
        min_version: CassSslTlsVersion,
    ) -> CassError;
}
