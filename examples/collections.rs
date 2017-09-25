// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;

use std::mem;
use std::ffi::CString;
use cassandra_cpp_sys::*;


fn insert_into_collections(session: &mut CassSession, key: &str, items: Vec<&str>) -> Result<(), CassError> {
    unsafe {
        let query = "INSERT INTO examples.collections (key, items) VALUES (?, ?);";

        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 2);
        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let collection = cass_collection_new(CASS_COLLECTION_TYPE_SET, 2);
        for item in items {
            cass_collection_append_string(collection, CString::new(item).unwrap().as_ptr());
        }
        cass_statement_bind_collection(statement, 1, collection);
        cass_collection_free(collection);

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

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

fn select_from_collections(session: &mut CassSession, key: &str) -> Result<(), CassError> {
    unsafe {
        let query = "SELECT items FROM examples.collections WHERE key = ?";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 1);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let iterator = cass_iterator_from_result(result);

                if cass_iterator_next(iterator) == cass_true {
                    let row = cass_iterator_get_row(iterator);
                    let value = cass_row_get_column(row, 0);
                    let items_iterator = cass_iterator_from_collection(value);

                    while cass_iterator_next(items_iterator) == cass_true {
                        let mut item = mem::zeroed();
                        let mut item_length = mem::zeroed();
                        cass_value_get_string(cass_iterator_get_value(items_iterator),
                                              &mut item,
                                              &mut item_length);
                        println!("item: {:?}", raw2utf8(item, item_length));
                    }
                    cass_iterator_free(items_iterator);
                };
                cass_iterator_free(iterator);
                cass_result_free(result);
                Ok(())
            }
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

fn main() {
    unsafe {
        let cluster = &mut *create_cluster();
        let session = &mut *cass_session_new();

        let items: Vec<&str> = vec!["apple", "orange", "banana", "mango"];

        cass_cluster_set_protocol_version(cluster, 2);

        connect_session(session, cluster).unwrap();
        execute_query(session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '1' };")
            .unwrap();

        execute_query(session,
                      "CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, PRIMARY KEY (key))")
            .unwrap();


        insert_into_collections(session, "test", items).unwrap();
        select_from_collections(session, "test").unwrap();

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);
        cass_session_free(session);
        cass_cluster_free(cluster);
    }
}
