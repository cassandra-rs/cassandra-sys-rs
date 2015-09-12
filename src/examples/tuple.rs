extern crate cql_bindgen;

use std::mem;
use std::ffi::CString;

use cql_bindgen::*;

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

fn insert_into_tuple(session: &mut CassSession, uuid_gen: &mut CassUuidGen) -> Result<(), CassError> {
    unsafe {
        let mut id = mem::zeroed();
        let mut id_str: [i8; 37] = mem::zeroed();

        let query = CString::new("INSERT INTO examples.tuples (id, item) VALUES (?, ?)").unwrap();

        let statement = &mut*cass_statement_new(query.as_ptr(), 2);
        cass_uuid_gen_time(uuid_gen, &mut id);
        cass_uuid_string(id, id_str[..].as_mut_ptr());

        let item = cass_tuple_new(2);

        cass_tuple_set_string(item, 0, id_str[..].as_mut_ptr());
        cass_tuple_set_int64(item, 1, id.time_and_version as i64);

        cass_statement_bind_uuid(statement, 0, id);
        cass_statement_bind_tuple(statement, 1, item);

        let future = &mut*cass_session_execute(session, statement);
        cass_future_wait(future);

        let rc = cass_future_error_code(future);
        if rc != CASS_OK {
            print_error(future);
        }

        cass_future_free(future);
        cass_statement_free(statement);
        cass_tuple_free(item);

        Ok(())
    }
}

fn select_from_tuple(session: &mut CassSession) -> Result<(), CassError> {
    unsafe {
        let query = CString::new("SELECT * FROM examples.tuples").unwrap();

        let statement = cass_statement_new(query.as_ptr(), 0);

        let future = &mut*cass_session_execute(session, statement);
        cass_future_wait(future);

        match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let rows = cass_iterator_from_result(result);

                while cass_iterator_next(rows) > 0 {
                    let mut id = mem::zeroed();
                    let mut id_str = mem::zeroed();
                    let row = cass_iterator_get_row(rows);
                    let id_value = cass_row_get_column_by_name(row, CString::new("id").unwrap().as_ptr());
                    let item_value = cass_row_get_column_by_name(row, CString::new("item").unwrap().as_ptr());
                    let item = cass_iterator_from_tuple(item_value);

                    cass_value_get_uuid(id_value, &mut id);
                    cass_uuid_string(id, &mut id_str);

                    print!("id {:?} ", &id_str);

                    while cass_iterator_next(item) > 0 {
                        let value = cass_iterator_get_value(item);

                        match cass_value_is_null(value) == 0 {
                            true => {
                                match cass_value_type(value) {
                                    CASS_VALUE_TYPE_VARCHAR => {
                                        let mut text = mem::zeroed();
                                        let mut text_length = mem::zeroed();
                                        cass_value_get_string(value, &mut text, &mut text_length);
                                        print!("{:?} ", raw2utf8(text,text_length));
                                    }
                                    CASS_VALUE_TYPE_BIGINT => {
                                        let mut i = mem::zeroed();
                                        cass_value_get_int64(value, &mut i);
                                        print!("{:?} ", i);
                                    }
                                    other_type => {
                                        print!("<invalid type {:?}> ", other_type);
                                    }
                                }

                            }
                            false => {
                                print!("<null> ");
                            }
                        }

                    }
                    println!("");
                }
                cass_result_free(result);
                cass_iterator_free(rows);
            }
            _ => {
                print_error(future);
            }
        }
        cass_future_free(future);
        cass_statement_free(statement);
        Ok(())
    }
}

fn main() {
    unsafe {
        let cluster = &mut*create_cluster();
        let mut session = &mut*cass_session_new();

        let uuid_gen = &mut*cass_uuid_gen_new();

        if connect_session(session, cluster) != CASS_OK {
            cass_cluster_free(cluster);
            cass_session_free(session);
            panic!();
        }

        execute_query(session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' }").unwrap();

        execute_query(session, "CREATE TABLE IF NOT EXISTS examples.tuples (id timeuuid, item frozen<tuple<text, bigint>>, PRIMARY KEY(id))").unwrap();

        insert_into_tuple(session, uuid_gen).unwrap();
        select_from_tuple(session).unwrap();

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

        cass_uuid_gen_free(uuid_gen);
    }
}
