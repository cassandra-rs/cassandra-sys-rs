// #![feature(plugin)]
// #![plugin(clippy)]
extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;
use std::ffi::CString;
use cassandra_cpp_sys::*;


#[derive(Copy,Clone)]
struct Basic {
    bln: cass_bool_t,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn prepare_query<'a>(session: &mut CassSession, query: &str) -> Result<&'a CassPrepared, CassError> {
    unsafe {
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

fn insert_into_basic(session: &mut CassSession, prepared: &CassPrepared, key: &str, basic: &Basic)
                     -> Result<(), CassError> {
    unsafe {
        let statement = cass_prepared_bind(prepared);
        cass_statement_bind_string_by_name(statement,
                                           CString::new("key").unwrap().as_ptr(),
                                           CString::new(key).unwrap().as_ptr());
        cass_statement_bind_bool_by_name(statement, CString::new("bln").unwrap().as_ptr(), basic.bln);
        cass_statement_bind_float_by_name(statement, CString::new("flt").unwrap().as_ptr(), basic.flt);
        cass_statement_bind_double_by_name(statement, CString::new("dbl").unwrap().as_ptr(), basic.dbl);
        cass_statement_bind_int32_by_name(statement, CString::new("i32").unwrap().as_ptr(), basic.i32);
        cass_statement_bind_int64_by_name(statement, CString::new("i64").unwrap().as_ptr(), basic.i64);

        let future = &mut *cass_session_execute(session, statement);

        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                println!("Bind by name insert succeeded");
                Ok(())
            }
            err => {
                print_error(future);
                Err(err)
            }
        };

        cass_future_free(future);
        cass_statement_free(statement);

        result
    }
}

fn select_from_basic<'a>(session: &mut CassSession, prepared: &CassPrepared, key: &str, basic: Basic)
                         -> Result<Basic, CassError> {
    unsafe {
        let statement = cass_prepared_bind(prepared);
        cass_prepared_free(prepared);
        cass_statement_bind_string_by_name(statement,
                                           CString::new("key").unwrap().as_ptr(),
                                           CString::new(key).unwrap().as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let rc = cass_future_error_code(future);

        let result = match rc {
            CASS_OK => {
                let mut output = basic;
                let result = cass_future_get_result(future);
                let iterator = cass_iterator_from_result(result);

                if cass_iterator_next(iterator) == cass_true {
                    let row = cass_iterator_get_row(iterator);

                    cass_value_get_bool(cass_row_get_column_by_name(row, CString::new("bln").unwrap().as_ptr()),
                                        &mut output.bln);
                    cass_value_get_double(cass_row_get_column_by_name(row, CString::new("dbl").unwrap().as_ptr()),
                                          &mut output.dbl);
                    cass_value_get_float(cass_row_get_column_by_name(row, CString::new("flt").unwrap().as_ptr()),
                                         &mut output.flt);
                    cass_value_get_int32(cass_row_get_column_by_name(row, CString::new("i32").unwrap().as_ptr()),
                                         &mut output.i32);
                    cass_value_get_int64(cass_row_get_column_by_name(row, CString::new("i64").unwrap().as_ptr()),
                                         &mut output.i64);
                }
                cass_iterator_free(iterator);
                cass_result_free(result);

                Ok(basic)
            }
            _ => {
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
        let session = &mut *cass_session_new();
        let input = Basic {
            bln: cass_true,
            flt: 0.001,
            dbl: 0.0002,
            i32: 1,
            i64: 2,
        };

        let insert_query = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
        let select_query = "SELECT * FROM examples.basic WHERE key = ?";

        match connect_session(session, cluster) {
            Ok(()) => {
                execute_query(session,
                              "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                               \'SimpleStrategy\', \'replication_factor\': \'1\' };")
                    .unwrap();

                execute_query(session,
                              "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl \
                               double,i32 int, i64 bigint, PRIMARY KEY (key));")
                    .unwrap();

                match prepare_query(session, &insert_query) {
                    Ok(insert_prepared) => {
                        insert_into_basic(session, insert_prepared, "prepared_test", &input).unwrap();
                        cass_prepared_free(insert_prepared);
                        match prepare_query(session, &select_query) {
                            Ok(select_prepared) => {
                                let output = select_from_basic(session, select_prepared, "prepared_test", input)
                                    .unwrap();

                                assert!(input.bln.clone() == output.bln);
                                assert!(input.flt == output.flt);
                                assert!(input.dbl == output.dbl);
                                assert!(input.i32 == output.i32);
                                assert!(input.i64 == output.i64);
                            }
                            _ => panic!("couldn't query"),
                        }
                    }
                    _ => panic!("couldn't prepare query"),
                }
            }
            rc => {
                println!("Error: {:?}", rc);
            }
        }
        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);
        cass_cluster_free(&mut *cluster);
        cass_session_free(session);

    }
}
