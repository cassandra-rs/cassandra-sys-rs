// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;
use std::ffi::CString;

mod examples_util;
use examples_util::*;

use std::mem;
use std::ffi::CStr;

use cassandra_cpp_sys::*;

const NUM_CONCURRENT_REQUESTS: usize = 1000;
#[derive(Clone)]
struct Basic {
    bln: u32,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_paging(session: &mut CassSession, uuid_gen: &mut CassUuidGen) {
    unsafe {
        let query = "INSERT INTO paging (key, value) VALUES (?, ?);";
        let mut futures = Vec::with_capacity(NUM_CONCURRENT_REQUESTS);

        for i in 0..NUM_CONCURRENT_REQUESTS {
            let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 2);
            let mut key = mem::zeroed();

            cass_uuid_gen_time(uuid_gen, &mut key);

            cass_statement_bind_uuid(statement, 0, key);
            cass_statement_bind_string(statement, 1, CString::new(i.to_string()).unwrap().as_ptr());

            futures.push(&mut *cass_session_execute(session, statement));

            cass_statement_free(statement);
        }

        for future in futures {
            match cass_future_error_code(future) {
                CASS_OK => {}
                _ => print_error(future),
            }

            cass_future_free(future);
        }
    }
}

fn select_from_paging(session: &mut CassSession) {
    unsafe {
        let mut has_more_pages = true;
        let query = "SELECT * FROM paging";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 0);

        cass_statement_set_paging_size(statement, 100);

        while has_more_pages {
            let future = cass_session_execute(session, statement);

            match cass_future_error_code(future) {
                CASS_OK => {
                    let result = cass_future_get_result(future);
                    let iterator = cass_iterator_from_result(result);
                    cass_future_free(future);

                    while cass_iterator_next(iterator) == cass_true {
                        let row = &*cass_iterator_get_row(iterator);
                        let mut key_str: [i8; 37] = mem::zeroed();
                        let mut key = mem::zeroed();
                        let mut value = mem::zeroed();
                        let mut value_length = mem::zeroed();

                        cass_value_get_uuid(cass_row_get_column(row, 0), &mut key);
                        cass_uuid_string(key, key_str[..].as_mut_ptr());

                        cass_value_get_string(cass_row_get_column(row, 1), &mut value, &mut value_length);
                        println!("key: {:?} value: {:?}",
                                 CStr::from_ptr(key_str[..].as_ptr()),
                                 raw2utf8(value, value_length).unwrap());
                    }
                    match cass_result_has_more_pages(result) == cass_true {
                        true => {
                            cass_statement_set_paging_state(statement, result);
                        }
                        false => has_more_pages = false,
                    }
                    cass_iterator_free(iterator);
                    cass_result_free(result);
                }
                _ => {
                    print_error(&mut *future);
                }
            }
        }
        cass_statement_free(statement);
    }
}

fn main() {
    unsafe {
        let uuid_gen = &mut *cass_uuid_gen_new();
        let cluster = create_cluster();
        let session = &mut *cass_session_new();

        connect_session(session, cluster).unwrap();

        execute_query(session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '3' };")
            .unwrap();


        execute_query(session,
                      "CREATE TABLE IF NOT EXISTS examples.paging (key timeuuid, value text, PRIMARY KEY (key));")
            .unwrap();

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
