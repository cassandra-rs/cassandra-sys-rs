// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

use std::mem;
use std::ffi::CString;
use cassandra_cpp_sys::*;

#[derive(Clone)]
struct Basic {
    bln: cass_bool_t,
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
        println!("Error: {:?}", raw2utf8(message, message_length).unwrap());
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

fn insert_into_basic(session: &mut CassSession, key: &str, basic: &Basic) -> Result<(), CassError> {
    unsafe {
        let query = CString::new("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (:k, :b, :f, :d, \
                                  :i32, :i64);")
            .unwrap();
        let key = CString::new(key).unwrap();
        let statement = &mut *cass_statement_new(query.as_ptr(), 6);
        let k = CString::new("k").unwrap().as_ptr();
        let b = CString::new("b").unwrap().as_ptr();
        let f = CString::new("f").unwrap().as_ptr();
        let d = CString::new("d").unwrap().as_ptr();
        let i32 = CString::new("i32").unwrap().as_ptr();
        let i64 = CString::new("i64").unwrap().as_ptr();
        cass_statement_bind_string_by_name(statement, k, key.as_ptr());
        cass_statement_bind_bool_by_name(statement, b, basic.bln);
        cass_statement_bind_float_by_name(statement, f, basic.flt);
        cass_statement_bind_double_by_name(statement, d, basic.dbl);
        cass_statement_bind_int32_by_name(statement, i32, basic.i32);
        cass_statement_bind_int64_by_name(statement, i64, basic.i64);

        let future = &mut *cass_session_execute(session, statement);

        cass_future_wait(future);

        let rc = cass_future_error_code(future);
        let result = match cass_future_error_code(future) {
            CASS_OK => Ok(()),
            rc => {
                print_error(future);
                Err(rc)
            }
        };

        cass_future_free(future);
        cass_statement_free(statement);

        Ok(())
    }
}

fn select_from_basic(session: &mut CassSession, key: &str) -> Result<Basic, CassError> {
    unsafe {
        let key = CString::new(key).unwrap();
        let query = CString::new("SELECT * FROM examples.basic WHERE key = ?").unwrap();
        let mut output: Basic = mem::zeroed();

        let statement = &mut *cass_statement_new(query.as_ptr(), 1);

        cass_statement_bind_string_by_name(statement,
                                           CString::new("key").unwrap().as_ptr(),
                                           key.as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let rc = cass_future_error_code(future);


        match rc {
            CASS_OK => {
                let result = &*cass_future_get_result(future);
                let iterator = &mut *cass_iterator_from_result(result);

                if cass_iterator_next(iterator) == cass_true {
                    let row = &*cass_iterator_get_row(iterator);

                    cass_value_get_bool(cass_row_get_column_by_name(row, CString::new("BLN").unwrap().as_ptr()),
                                        &mut output.bln);
                    cass_value_get_double(cass_row_get_column_by_name(row, CString::new("dbl").unwrap().as_ptr()),
                                          &mut output.dbl);
                    cass_value_get_float(cass_row_get_column_by_name(row, CString::new("flt").unwrap().as_ptr()),
                                         &mut output.flt);
                    cass_value_get_int32(cass_row_get_column_by_name(row, CString::new("\"i32\"").unwrap().as_ptr()),
                                         &mut output.i32);
                    cass_value_get_int64(cass_row_get_column_by_name(row, CString::new("i64").unwrap().as_ptr()),
                                         &mut output.i64);
                    cass_result_free(result);
                    cass_iterator_free(iterator);
                }
            }
            rc => println!("{:?}", rc),

        }

        cass_future_free(future);
        cass_statement_free(statement);

        Ok(output)
    }
}

fn main() {
    unsafe {
        let cluster = &mut *create_cluster();
        let session = &mut *cass_session_new();
        let input = Basic {
            bln: cass_true,
            flt: 0.001f32,
            dbl: 0.0002,
            i32: 1,
            i64: 2,
        };

        let result = match connect_session(session, cluster) {
            CASS_OK => {}
            rc => {
                cass_cluster_free(cluster);
                cass_session_free(session);
            }
        };



        execute_query(session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '3' };")
            .unwrap();


        execute_query(session,
                      "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 \
                       int, i64 bigint, PRIMARY KEY (key));")
            .unwrap();


        insert_into_basic(session, "named_parameters", &input).unwrap();
        let output = select_from_basic(session, "named_parameters").unwrap();

        assert!(input.bln == output.bln);
        assert!(input.flt == output.flt);
        assert!(input.dbl == output.dbl);
        assert!(input.i32 == output.i32);
        assert!(input.i64 == output.i64);

        let close_future = &mut *cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

    }
}
