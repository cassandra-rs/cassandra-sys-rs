#![allow(dead_code)]

use cassandra_cpp_sys::*;

use std::mem;
use std::ffi::CString;

pub fn print_error(future: &mut CassFuture) {
    unsafe {
        let mut message = mem::zeroed();
        let mut message_length = mem::zeroed();
        cass_future_error_message(future, &mut message, &mut message_length);
        println!("Error: {:?}", raw2utf8(message, message_length));
    }
}

pub fn connect_session(session: &mut CassSession, cluster: &CassCluster) -> Result<(), CassError> {
    unsafe {
        let future = cass_session_connect(session, cluster);
        cass_future_wait(future);
        let err = cass_future_error_code(future);
        cass_future_free(future);
        match err {
            CASS_OK => Ok(()),
            rc => Err(rc),
        }
    }
}

pub fn create_cluster() -> &'static mut CassCluster {
    unsafe {
        let cluster = cass_cluster_new();
        let host = CString::new("127.0.0.1").unwrap();
        cass_cluster_set_contact_points(cluster, host.as_ptr());
        &mut *cluster
    }
}

pub fn execute_query(session: &mut CassSession, query: &str) -> Result<(), CassError> {
    unsafe {
        let cstring = CString::new(query);
        let statement = cass_statement_new(cstring.unwrap().as_ptr(), 0);
        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);
        cass_future_error_code(future);
        let result = match cass_future_error_code(future) {
            CASS_OK => Ok(()),
            rc => {
                print_error(future);
                Err(rc)
            }
        };
        cass_future_free(future);
        cass_statement_free(statement);
        result
    }
}
