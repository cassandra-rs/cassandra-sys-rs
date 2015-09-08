extern crate cql_bindgen;
use std::ffi::CString;

use cql_bindgen::*;

use std::mem;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

fn print_error(future:*mut CassFuture) {
  let mut message = unsafe{mem::zeroed()};
  let mut message_length = unsafe{mem::zeroed()};
  unsafe{cass_future_error_message(future, &mut message, &mut message_length)};
   println!("Error: {:?}", unsafe{raw2utf8(message,message_length)});
}

fn create_cluster() -> *mut CassCluster {
    let cluster = unsafe{cass_cluster_new()};
    let host = CString::new("127.0.0.1").unwrap();
    unsafe{cass_cluster_set_contact_points(cluster,host.as_ptr() as *const i8)};
    cluster 
}

fn connect_session(session:&mut CassSession, cluster:&CassCluster) -> CassError {
    let future = unsafe{cass_session_connect(session,cluster)};
    unsafe{cass_future_wait(future)};
    let err = unsafe{cass_future_error_code(future)};
    unsafe{cass_future_free(future)};
    err
}

fn execute_query(session: &mut CassSession, query: &str) -> CassError {
    let query = CString::new(query).unwrap();
    let statement = unsafe{cass_statement_new(query.as_ptr(), 0)};
    let future = unsafe{cass_session_execute(session,statement)};
    unsafe{cass_future_wait(future)};
    unsafe{cass_future_error_code(future)};
    let rc = unsafe{cass_future_error_code(future)};
    if rc != CASS_OK {
        print_error(future);
    }
    unsafe{cass_future_free(future)};
    unsafe{cass_statement_free(statement)};
    rc
}

fn insert_into_async(session: &mut CassSession, key: &str) {
  //CassError rc = CASS_OK;
  //CassStatement* statement = NULL;
  let query = "INSERT INTO async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
  let futures = &mut Vec::with_capacity(NUM_CONCURRENT_REQUESTS);


//  size_t i;
  for i in 0..NUM_CONCURRENT_REQUESTS {
     //char key_buffer[64];
      let query = CString::new(query).unwrap();
    let statement = unsafe{cass_statement_new(query.as_ptr(), 6)};

    let key = format!("{}{}", key, i);
    let key = CString::new(key).unwrap();
    unsafe{cass_statement_bind_string(statement, 0, key.as_ptr())};
    unsafe{cass_statement_bind_bool(statement, 1, if i % 2 == 0 {cass_true} else {cass_false})};
    unsafe{cass_statement_bind_float(statement, 2, i as f32 / 2.0f32)};
    unsafe{cass_statement_bind_double(statement, 3, i as f64 / 200.0)};
    unsafe{cass_statement_bind_int32(statement, 4, i as i32 * 10)};
    unsafe{cass_statement_bind_int64(statement, 5, i as i64 * 100)};

    futures.push(unsafe{cass_session_execute(session, statement)});

    unsafe{cass_statement_free(statement)};
  }

  for i in 0..NUM_CONCURRENT_REQUESTS {
    let future = futures[i];

    unsafe{cass_future_wait(future)};

    let rc = unsafe{cass_future_error_code(future)};
    if rc != CASS_OK {
      print_error(future);
    }

    unsafe{cass_future_free(future)};
  }
}

pub fn main() {
  let cluster = create_cluster();
  let session = unsafe{&mut*cass_session_new()};

  if unsafe{connect_session(&mut*session, &mut*cluster)} != CASS_OK {
    unsafe{cass_cluster_free(cluster)};
    unsafe{cass_session_free(session)};
  }

  execute_query(&mut*session,
                "CREATE KEYSPACE examples WITH replication = { \
                           'class': 'SimpleStrategy', 'replication_factor': '3' };");


  execute_query(&mut*session,
                "CREATE TABLE examples.async (key text, \
                                              bln boolean, \
                                              flt float, dbl double,\
                                              i32 int, i64 bigint, \
                                              PRIMARY KEY (key));");

  execute_query(session, "USE examples");

  insert_into_async(session, "test");

  let close_future = unsafe{cass_session_close(session)};
  unsafe{cass_future_wait(close_future)};
  unsafe{cass_future_free(close_future)};

  unsafe{cass_cluster_free(cluster)};
  unsafe{cass_session_free(session)};

}