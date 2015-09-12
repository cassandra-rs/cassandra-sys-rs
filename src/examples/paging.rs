#![feature(plugin)]
#![plugin(clippy)]
#![allow(float_cmp)]

extern crate cql_bindgen;
extern crate num;

use std::mem;
use std::ffi::CString;
use std::ffi::CStr;

use cql_bindgen::*;

const NUM_CONCURRENT_REQUESTS:usize = 1000;
#[derive(Clone)]
struct Basic {
    bln: u32,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn print_error(future: &mut CassFuture) {
    unsafe {
        let mut message = mem::zeroed();
        let mut message_length = mem::zeroed();
        cass_future_error_message(future, &mut message, &mut message_length);
        println!("Error: {:?}", raw2utf8(message,message_length));
    }
}

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

fn execute_query(session: &mut CassSession, query: &str) -> Result<(), CassError> {
    unsafe {
        let query = CString::new(query).unwrap();
        let statement = cass_statement_new(query.as_ptr(), 0);
        let future = &mut*cass_session_execute(session, statement);
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

fn insert_into_paging(session: &mut CassSession, uuid_gen: &mut CassUuidGen) {
    unsafe {
        let query = CString::new("INSERT INTO paging (key, value) VALUES (?, ?);").unwrap();

        let mut futures: Vec<&mut CassFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS);
        for i in 0..NUM_CONCURRENT_REQUESTS {
            let mut key = mem::zeroed();
            let statement = cass_statement_new(query.as_ptr(), 2);

            cass_uuid_gen_time(uuid_gen, &mut key);
            cass_statement_bind_uuid(statement, 0, key);

            let value_buffer = CString::new(i.to_string()).unwrap();
            cass_statement_bind_string(statement, 1, value_buffer.as_ptr());

            futures.push(&mut* cass_session_execute(session, statement));

            cass_statement_free(statement);
        }

        for future in futures {
            let rc = cass_future_error_code(future);
            if rc != CASS_OK {
                print_error(future);
            }

            cass_future_free(future);
        }
    }
}

fn select_from_paging(session: &mut CassSession) {
    unsafe {
        let mut has_more_pages = true;
        let query = CString::new("SELECT * FROM paging").unwrap();
        let statement = cass_statement_new(query.as_ptr(), 0);

        cass_statement_set_paging_size(statement, 100);

        while has_more_pages {
            let future = cass_session_execute(session, statement);

            if cass_future_error_code(future) != 0 {
                print_error(&mut*future);
                break;
            }

            let result = cass_future_get_result(future);
            let iterator = cass_iterator_from_result(result);
            cass_future_free(future);

            while cass_iterator_next(iterator) > 0 {
                let row = &* cass_iterator_get_row(iterator);
                //const CASS_UUID_STRING_LENGTH:usize = 37;
                let mut key_str: [i8; 37] = mem::zeroed();
                let mut key = mem::zeroed();
                let mut value = mem::zeroed();
                let mut value_length = mem::zeroed();

                cass_value_get_uuid(cass_row_get_column(row, 0), &mut key);
                cass_uuid_string(key, key_str[..].as_mut_ptr());

                cass_value_get_string(cass_row_get_column(row, 1), &mut value, &mut value_length);
                let cstr_key = CStr::from_ptr(key_str[..].as_ptr());
                println!("key: '{:?}' value: '{:?}'", cstr_key, raw2utf8(value,value_length));
            }

            has_more_pages = cass_result_has_more_pages(result) > 0;

            if has_more_pages {
                cass_statement_set_paging_state(statement, result);
            }

            cass_iterator_free(iterator);
            cass_result_free(result);

        }
        cass_statement_free(statement);
    }
}

fn main() {
    unsafe {
        let uuid_gen = &mut* cass_uuid_gen_new();
        let cluster = &mut* create_cluster();
        let session = &mut* cass_session_new();

        if connect_session(session, cluster) != CASS_OK {
            cass_cluster_free(cluster);
            cass_session_free(session);
            panic!();
        }

        execute_query(session,
                "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \
                           'class': 'SimpleStrategy', 'replication_factor': '3' };").unwrap();


        execute_query(session,
                "CREATE TABLE IF NOT EXISTS examples.paging (key timeuuid, \
                                               value text, \
                                               PRIMARY KEY (key));").unwrap();

        execute_query(session, "USE examples").unwrap();

        insert_into_paging(session, uuid_gen);
        select_from_paging(session);

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_uuid_gen_free(uuid_gen);
        cass_cluster_free(cluster);
        cass_session_free(session);

    }
}
