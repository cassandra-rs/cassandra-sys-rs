#![feature(plugin)]
#![plugin(clippy)]
#![allow(float_cmp)]

extern crate cql_bindgen;
extern crate num;

use std::mem;
use std::ffi::CString;

use cql_bindgen::*;

struct Pair {
  key:String,
  value:i32
}

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

fn insert_into_maps(session: &mut CassSession, key:&str, items:Vec<Pair>) -> Result<(), CassError> {unsafe{
  let query = CString::new("INSERT INTO examples.maps (key, items) VALUES (?, ?);").unwrap();
    let key = CString::new(key).unwrap();
  let statement = cass_statement_new(query.as_ptr(), 2);

  cass_statement_bind_string(statement, 0, key.as_ptr());

  let collection = cass_collection_new(CASS_COLLECTION_TYPE_MAP, 5);
  for item in items {
      let item_key = CString::new(item.key).unwrap();
    cass_collection_append_string(collection, item_key.as_ptr());
    cass_collection_append_int32(collection, item.value);
  }
  cass_statement_bind_collection(statement, 1, collection);
  cass_collection_free(collection);

  let future = &mut*cass_session_execute(session, statement);
  cass_future_wait(future);

  let rc = cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(future);
  }

  cass_future_free(future);
  cass_statement_free(statement);

  Ok(())
}}

fn select_from_maps(session:&mut CassSession, key:&str) -> Result<(), CassError> {unsafe{
  let query = CString::new("SELECT items FROM examples.maps WHERE key = ?").unwrap();
   let key = CString::new(key).unwrap();
  let statement = cass_statement_new(query.as_ptr(), 1);

  cass_statement_bind_string(statement, 0, key.as_ptr());

  let future = &mut*cass_session_execute(session, statement);
  cass_future_wait(future);

  let rc = cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(future);
  } else {
    let result = cass_future_get_result(future);

    if cass_result_row_count(result) > 0 {
      let row = cass_result_first_row(result);

      let iterator
          = cass_iterator_from_map(
              cass_row_get_column(row, 0));

      while cass_iterator_next(iterator) > 0 {
          let mut item_key = mem::zeroed();
          let mut item_key_length = mem::zeroed();
          let mut value = mem::zeroed();
          cass_value_get_string(cass_iterator_get_map_key(iterator), &mut item_key, &mut item_key_length);
          cass_value_get_int32(cass_iterator_get_map_value(iterator), &mut value);
          
          println!("item: '{:?}' : {:?}", raw2utf8(item_key, item_key_length), value);
      }
      cass_iterator_free(iterator);
    }

    cass_result_free(result);
  }

  cass_future_free(future);
  cass_statement_free(statement);
Ok(())
}}

fn main() {unsafe{
  let items = vec!(
      Pair{key: "apple".to_owned(), value:1 }, 
      Pair{key:"orange".to_owned(), value:2 }, 
      Pair{key:"banana".to_owned(), value:3 }, 
      Pair{key:"mango".to_owned(), value:4 }
    );

    let cluster = &mut*create_cluster();
    let session = &mut*cass_session_new();
  if connect_session(session, cluster) != CASS_OK {
    cass_cluster_free(cluster);
    cass_session_free(session);
  }

  execute_query(session,
                "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \
                'class': 'SimpleStrategy', 'replication_factor': '3' };").unwrap();

  execute_query(session,
                "CREATE TABLE IF NOT EXISTS examples.maps (key text, \
                items map<text, int>, \
                PRIMARY KEY (key))").unwrap();


  insert_into_maps(session, "test", items).unwrap();
  select_from_maps(session, "test").unwrap();

  let close_future = cass_session_close(session);
  cass_future_wait(close_future);
  cass_future_free(close_future);

  cass_cluster_free(cluster);
  cass_session_free(session);
}}