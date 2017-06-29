// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;

use std::mem;
use std::ffi::CStr;
use std::ffi::CString;

use std::str;

use cassandra_cpp_sys::*;

const CASS_UUID_STRING_LENGTH: usize = 37;

fn insert_into_log(session: &mut CassSession, key: &str, time: CassUuid, entry: &str) -> Result<(), CassError> {
    unsafe {
        let query = "INSERT INTO examples.log (key, time, entry) VALUES (?, ?, ?);";

        let statement: *mut CassStatement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 3);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());
        cass_statement_bind_uuid(statement, 1, time);
        cass_statement_bind_string(statement, 2, CString::new(entry).unwrap().as_ptr());

        let future = cass_session_execute(session, statement);

        cass_future_wait(future);

        let rc = cass_future_error_code(future);
        cass_future_free(future);
        cass_statement_free(statement);

        match rc {
            CASS_OK => Ok(()),
            err => {
                let mut message = mem::zeroed();
                let mut message_length = mem::zeroed();
                cass_future_error_message(future, &mut message, &mut message_length);
                println!("{:?}", raw2utf8(message, message_length));
                Err(err)
            }
        }
    }
}

fn select_from_log(session: &mut CassSession, key: &str) -> Result<(), CassError> {
    unsafe {
        let query = "SELECT * FROM examples.log WHERE key = ?";

        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 1);

        cass_statement_bind_string(statement, 0, CString::new(key).unwrap().as_ptr());

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        let result = match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let iterator = cass_iterator_from_result(result);

                while cass_iterator_next(iterator) == cass_true {
                    let row = cass_iterator_get_row(iterator);
                    let mut key_length = mem::zeroed();
                    let mut time: CassUuid = mem::zeroed();
                    let mut entry = mem::zeroed();
                    let mut entry_length = mem::zeroed();
                    let mut time_str: [i8; CASS_UUID_STRING_LENGTH] = [0; CASS_UUID_STRING_LENGTH];

                    cass_value_get_string(cass_row_get_column(row, 0),
                                          &mut CString::new(key).unwrap().as_ptr(),
                                          &mut key_length);
                    cass_value_get_uuid(cass_row_get_column(row, 1), &mut time);
                    cass_value_get_string(cass_row_get_column(row, 2), &mut entry, &mut entry_length);
                    let mut output: i8 = mem::zeroed();
                    cass_uuid_string(time, &mut output);
                    let output = CStr::from_ptr(&output);
                    println!("{:?}", str::from_utf8(output.to_bytes()).unwrap());

                    cass_uuid_string(time, time_str[..].as_mut_ptr());
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

pub fn main() {
    unsafe {
        let cluster = create_cluster();
        let session = &mut *cass_session_new();
        let uuid_gen = cass_uuid_gen_new();
        let mut uuid = mem::zeroed();

        match connect_session(session, cluster) {
            Ok(()) => {
                execute_query(&mut *session,
                              "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': \
                               'SimpleStrategy', 'replication_factor': '1' };")
                    .unwrap();
                execute_query(&mut *session,
                              "CREATE TABLE IF NOT EXISTS examples.log (key text, time timeuuid, entry text, PRIMARY \
                               KEY (key, time));")
                    .unwrap();

                cass_uuid_gen_time(uuid_gen, &mut uuid);

                insert_into_log(&mut *session, "test", uuid, "Log entry #1").unwrap();

                cass_uuid_gen_time(uuid_gen, &mut uuid);
                insert_into_log(&mut *session, "test", uuid, "Log entry #2").unwrap();

                cass_uuid_gen_time(uuid_gen, &mut uuid);
                insert_into_log(&mut *session, "test", uuid, "Log entry #3").unwrap();

                cass_uuid_gen_time(uuid_gen, &mut uuid);
                insert_into_log(&mut *session, "test", uuid, "Log entry #4").unwrap();

                select_from_log(&mut *session, "test").unwrap();

                let close_future = cass_session_close(session);
                cass_future_wait(close_future);
                cass_future_free(close_future);
            }
            rc => panic!("{:?}", rc),
        }
        cass_uuid_gen_free(uuid_gen);
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
