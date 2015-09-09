#![feature(plugin)]
#![plugin(clippy)]
#![allow(float_cmp)]
#[macro_use]
extern crate log;
extern crate cql_bindgen;
extern crate num;
extern crate libc;
extern crate env_logger;

use std::mem;
use std::ptr;
use std::env;

use libc::types::common::c95::c_void;
use std::ffi::CString;

use cql_bindgen::*;

fn create_cluster() -> *mut CassCluster {
    unsafe {
        let cluster = cass_cluster_new();
        let host = CString::new("127.0.0.1").unwrap();
        cass_cluster_set_contact_points(cluster, host.as_ptr());
        cluster
    }
}

fn connect_session(session: &mut CassSession, cluster: &CassCluster) -> CassError {
    unsafe {
        let future = cass_session_connect(session, cluster);
        cass_future_wait(future);
        let err = cass_future_error_code(future);
        cass_future_free(future);
        err
    }
}


extern "C" fn on_log(message: *const CassLogMessage, data: *mut c_void) {
    unsafe {
        let _ = data;
        let message = &*message;
        let bytes = mem::transmute::<[i8; 256], [u8; 256]>(message.message);
        let message_txt = std::str::from_utf8(&bytes).unwrap();
        info!(target: "cass_log", "{:?}.{:?} [{:?}] ({:?}:{:?}:{:?}) {:?}",
        message.time_ms / 1000,
        message.time_ms % 1000,
        message.severity,
        message.file,
        message.line,
        message.function,
        message_txt
    );
    }
}

fn main() {
    unsafe {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
  /* Log configuration *MUST* be done before any other driver call */
        cass_log_set_level(CASS_LOG_INFO);
        cass_log_set_callback(Some(on_log), ptr::null_mut());

        let cluster = &mut*create_cluster();
        let session = &mut*cass_session_new();

        if connect_session(session, cluster) != CASS_OK {
            cass_cluster_free(cluster);
            cass_session_free(session);
        }

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

    }
}
