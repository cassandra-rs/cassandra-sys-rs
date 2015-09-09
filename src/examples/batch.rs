#![feature(plugin)]
#![plugin(clippy)]

extern crate cql_bindgen;
extern crate num;

use std::mem;
use std::ffi::CString;

use cql_bindgen::*;


struct Pair {
    key: String,
    value: String,
}

fn print_error(future: *mut CassFuture) {
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

fn execute_query(session: &mut CassSession, query: &str) -> CassError {
    unsafe {
        let query = CString::new(query).unwrap();
        let statement = cass_statement_new(query.as_ptr(), 0);
        let future = cass_session_execute(session, statement);
        cass_future_wait(future);
        cass_future_error_code(future);
        let rc = cass_future_error_code(future);
        if rc != CASS_OK {
            print_error(future);
        }
        cass_future_free(future);
        cass_statement_free(statement);
        rc
    }
}

fn prepare_insert_into_batch<'a>(session: &mut CassSession, mut prepared: &'a CassPrepared)
                                 -> Result<&'a CassPrepared, CassError> {
    unsafe {
        let query = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";

        let query = CString::new(query).unwrap();
        let future = cass_session_prepare(session, query.as_ptr());
        cass_future_wait(future);
        let rc = cass_future_error_code(future);
        match rc {
            CASS_OK => {
                prepared = &*cass_future_get_prepared(future);
                cass_future_free(future);
                Ok(prepared)
            }
            _ => {
                print_error(future);
                cass_future_free(future);
                Err(rc)
            }
        }
    }
}

fn insert_into_batch_with_prepared(session: &mut CassSession, prepared: &CassPrepared, pairs: Vec<Pair>)
                                   -> Result<(), CassError> {
    unsafe {
        let batch = cass_batch_new(CASS_BATCH_TYPE_LOGGED);

        for pair in pairs {
            let statement = cass_prepared_bind(prepared);
            let key = CString::new(pair.key).unwrap();
            let value = CString::new(pair.value).unwrap();
            cass_statement_bind_string(statement, 0, key.as_ptr());
            cass_statement_bind_string(statement, 1, value.as_ptr());
            cass_batch_add_statement(batch, statement);
            cass_statement_free(statement);
        }

        let statement1 = CString::new("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')")
            .unwrap();
        let statement1 = cass_statement_new(statement1.as_ptr(), 0);
        cass_batch_add_statement(batch, statement1);
        cass_statement_free(statement1);

        let statement2 = CString::new("INSERT INTO examples.pairs (key, value) VALUES (?, ?)")
            .unwrap();
        let statement2 = cass_statement_new(statement2.as_ptr(), 2);
        let key = CString::new("d").unwrap();
        let value = CString::new("4").unwrap();

        cass_statement_bind_string(statement2, 0, key.as_ptr());
        cass_statement_bind_string(statement2, 1, value.as_ptr());
        cass_batch_add_statement(batch, statement2);
        cass_statement_free(statement2);

        let future = cass_session_execute_batch(session, batch);
        cass_future_wait(future);

        let rc = cass_future_error_code(future);
        let result = match rc {
            CASS_OK => Ok(()),
            _ => {
                print_error(future);
                Err(rc)
            }
        };

        cass_future_free(future);
        cass_batch_free(batch);

        result
    }
}


pub fn main() {
    unsafe {
        let cluster = &mut*create_cluster();
        let session = &mut*cass_session_new();

        let pairs = vec!(
            Pair{key:"a".to_owned(), value:"1".to_owned()},
            Pair{key:"b".to_owned(), value:"2".to_owned()}
        );

        if connect_session(session, cluster) != CASS_OK {
            cass_cluster_free(cluster);
            cass_session_free(session);
        }

        execute_query(session,
                      "CREATE KEYSPACE examples WITH replication = { \'class\': \'SimpleStrategy\', \
                       \'replication_factor\': \'3\' };");


        execute_query(session, "CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));");
        let prepared = mem::zeroed();
        match prepare_insert_into_batch(session, prepared) {
            Ok(prepared) => {
                insert_into_batch_with_prepared(session, prepared, pairs).unwrap();
                cass_prepared_free(prepared);
            }
            Err(rc) => println!("err: {:?}", rc),
        }

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

    }
}
