// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;
mod examples_util;
use examples_util::*;

use cassandra_cpp_sys::*;

use std::ffi::CString;

#[derive(Debug)]
struct Basic {
    bln: cass_bool_t,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut CassSession, key: &str, basic: &mut Basic) -> Result<(), CassError> {
    unsafe {
        let query = CString::new("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, \
                                  ?);");
        let statement = cass_statement_new(query.unwrap().as_ptr(), 6);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());
        cass_statement_bind_bool(statement, 1, basic.bln.into());
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

fn select_from_basic(session: &mut CassSession, key: &str, basic: &mut Basic) -> Result<(), CassError> {
    unsafe {
        let query = "SELECT * FROM examples.basic WHERE key = ?";
        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 1);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let future = cass_session_execute(session, statement);
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let iterator = cass_iterator_from_result(result);
                match cass_iterator_next(iterator) {
                    cass_true => {
                        let row = cass_iterator_get_row(iterator);

                        let ref mut b_bln = basic.bln;
                        let ref mut b_dbl = basic.dbl;
                        let ref mut b_flt = basic.flt;
                        let ref mut b_i32 = basic.i32;
                        let ref mut b_i64 = basic.i64;

                        cass_value_get_bool(cass_row_get_column(row, 1), b_bln);
                        cass_value_get_double(cass_row_get_column(row, 2), b_dbl);
                        cass_value_get_float(cass_row_get_column(row, 3), b_flt);
                        cass_value_get_int32(cass_row_get_column(row, 4), b_i32);
                        cass_value_get_int64(cass_row_get_column(row, 5), b_i64);

                        cass_statement_free(statement);
                        cass_iterator_free(iterator);
                    }
                    cass_false => {}
                }
                cass_result_free(result);
                Ok(())
            }
            rc => Err(rc),
        };
        cass_future_free(future);
        result
    }
}

pub fn main() {
    unsafe {
        let cluster = create_cluster();
        let session = &mut *cass_session_new();

        let input = &mut Basic {
            bln: cass_true,
            flt: 0.001f32,
            dbl: 0.0002f64,
            i32: 1,
            i64: 2,
        };

        match connect_session(session, cluster) {
            Ok(()) => {
                let output = &mut Basic {
                    bln: cass_false,
                    flt: 0f32,
                    dbl: 0f64,
                    i32: 0,
                    i64: 0,
                };
                execute_query(session,
                              "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': \
                               'SimpleStrategy', 'replication_factor': '1' };")
                    .unwrap();
                execute_query(session,
                              "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl \
                               double, i32 int, i64 bigint, PRIMARY KEY (key));")
                    .unwrap();

                insert_into_basic(session, "test", input).unwrap();
                select_from_basic(session, "test", output).unwrap();

                println!("{:?}", input);
                println!("{:?}", output);

                assert!(input.bln == output.bln);
                assert!(input.flt == output.flt);
                assert!(input.dbl == output.dbl);
                assert!(input.i32 == output.i32);
                assert!(input.i64 == output.i64);

                let close_future = cass_session_close(session);

               cass_future_wait(close_future);
                cass_future_free(close_future);
            }
            err => println!("Error: {:?}", err),
        }
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
