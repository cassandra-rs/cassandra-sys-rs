// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;
use std::mem;
use std::ffi::CString;

use cassandra_cpp_sys::*;

struct Pair {
    key: String,
    value: i32,
}

fn insert_into_maps(session: &mut CassSession, key: &str, items: Vec<Pair>) -> Result<(), CassError> {
    unsafe {
        let query = "INSERT INTO examples.maps (key, items) VALUES (?, ?);";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 2);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let collection = cass_collection_new(CASS_COLLECTION_TYPE_MAP, 5);
        for item in items {
            cass_collection_append_string(collection, CString::new(item.key).unwrap().as_ptr());
            cass_collection_append_int32(collection, item.value);
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

        //        cass_future_free(future);
        //        cass_statement_free(statement);

        Ok(())
    }
}

fn select_from_maps(session: &mut CassSession, key: &str) -> Result<(), CassError> {
    unsafe {
        let query = "SELECT items FROM examples.maps WHERE key = ?";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 1);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);

                if cass_result_row_count(result) > 0 {
                    let row = cass_result_first_row(result);

                    let iterator = cass_iterator_from_map(cass_row_get_column(row, 0));

                    while cass_iterator_next(iterator) == cass_true {
                        let mut item_key = mem::zeroed();
                        let mut item_key_length = mem::zeroed();
                        let mut value = mem::zeroed();
                        cass_value_get_string(cass_iterator_get_map_key(iterator),
                                              &mut item_key,
                                              &mut item_key_length);
                        cass_value_get_int32(cass_iterator_get_map_value(iterator), &mut value);

                        println!("item: '{:?}' : {:?}",
                                 raw2utf8(item_key, item_key_length),
                                 value);
                    }
                    cass_iterator_free(iterator);
                    cass_result_free(result);
                }
                Ok(())
            }
            rc => Err(rc),
        };
        cass_future_free(future);
        cass_statement_free(statement);
        result
    }
}

fn main() {
    unsafe {
        let items = vec![Pair {
                             key: "apple".to_owned(),
                             value: 1,
                         },
                         Pair {
                             key: "orange".to_owned(),
                             value: 2,
                         },
                         Pair {
                             key: "banana".to_owned(),
                             value: 3,
                         },
                         Pair {
                             key: "mango".to_owned(),
                             value: 4,
                         }];

        let cluster = create_cluster();
        let session = &mut *cass_session_new();

        connect_session(session, &cluster).unwrap();

        execute_query(session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '3' };")
            .unwrap();

        execute_query(session,
                      "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))")
            .unwrap();


        insert_into_maps(session, "test", items).unwrap();
        //        select_from_maps(session, "test").unwrap();

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
