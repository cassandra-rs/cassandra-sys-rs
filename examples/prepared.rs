// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;
use std::ffi::CString;
use std::mem;

use cassandra_cpp_sys::*;

#[derive(Debug)]
struct Basic {
    bln: cass_bool_t,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut CassSession, key: &str, basic: &Basic) -> Result<(), CassError> {
    unsafe {
        let query = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 6);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());
        cass_statement_bind_bool(statement, 1, basic.bln);
        cass_statement_bind_float(statement, 2, basic.flt);
        cass_statement_bind_double(statement, 3, basic.dbl);
        cass_statement_bind_int32(statement, 4, basic.i32);
        cass_statement_bind_int64(statement, 5, basic.i64);

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

fn prepare_select_from_basic(session: &mut CassSession) -> Result<&CassPrepared, CassError> {
    unsafe {
        let query = "SELECT * FROM examples.basic WHERE key = ?";

        let future = &mut *cass_session_prepare(session, CString::new(query).unwrap().as_ptr());
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let prepared = &*cass_future_get_prepared(future);
                Ok(prepared)
            }
            rc => {
                print_error(future);
                Err(rc)
            }
        };
        cass_future_free(future);
        result
    }
}

fn select_from_basic(session: &mut CassSession, prepared: &CassPrepared, key: &str, basic: &mut Basic)
                     -> Result<(), CassError> {
    unsafe {
        let statement = cass_prepared_bind(prepared);
        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let iterator = cass_iterator_from_result(result);

                if cass_iterator_next(iterator) == cass_true {
                    let row = cass_iterator_get_row(iterator);

                    cass_value_get_bool(cass_row_get_column(row, 1), &mut basic.bln);
                    cass_value_get_double(cass_row_get_column(row, 2), &mut basic.dbl);
                    cass_value_get_float(cass_row_get_column(row, 3), &mut basic.flt);
                    cass_value_get_int32(cass_row_get_column(row, 4), &mut basic.i32);
                    cass_value_get_int64(cass_row_get_column(row, 5), &mut basic.i64);
                    println!("Got row: {:?}", basic);
                }
                cass_result_free(result);
                cass_iterator_free(iterator);

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
        let cluster = create_cluster();
        let session = cass_session_new();
        let input = Basic {
            bln: cass_true,
            flt: 0.001,
            dbl: 0.0002,
            i32: 1,
            i64: 2,
        };
        let mut output = mem::zeroed();

        connect_session(&mut *session, cluster).unwrap();

        execute_query(&mut *session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '3' };")
            .unwrap();

        execute_query(&mut *session,
                      "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 \
                       int, i64 bigint, PRIMARY KEY (key));")
            .unwrap();

        insert_into_basic(&mut *session, "prepared_test", &input).unwrap();

        let prepared = prepare_select_from_basic(&mut *session).unwrap();
        select_from_basic(&mut *session, &prepared, "prepared_test", &mut output).unwrap();

        assert!(input.bln == output.bln);
        assert!(input.flt == output.flt);
        assert!(input.dbl == output.dbl);
        assert!(input.i32 == output.i32);
        assert!(input.i64 == output.i64);

        cass_prepared_free(prepared);

        let close_future = cass_session_close(session);

        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
