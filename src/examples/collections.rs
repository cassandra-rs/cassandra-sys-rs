#![feature(plugin)]
#![plugin(clippy)]
#![allow(float_cmp)]

extern crate cql_bindgen;
extern crate num;

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

fn insert_into_collections(session:&mut CassSession, key:&str, items: Vec<&str>) -> Result<(), CassError> {unsafe{
    let query = CString::new("INSERT INTO examples.collections (key, items) VALUES (?, ?);").unwrap();
    let key = CString::new(key).unwrap();

    let statement = cass_statement_new(query.as_ptr(), 2);
    cass_statement_bind_string(statement, 0, key.as_ptr());

  let collection = cass_collection_new(CASS_COLLECTION_TYPE_SET, 2);
  for item in items {
      let item = CString::new(item).unwrap();
    cass_collection_append_string(collection, item.as_ptr());
  }
  cass_statement_bind_collection(statement, 1, collection);
  cass_collection_free(collection);

  let future = &mut*cass_session_execute(session, statement);
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
}}

fn select_from_collections(session: &mut CassSession, key: &str) -> Result<(), CassError> {unsafe{
   let query = CString::new("SELECT items FROM examples.collections WHERE key = ?").unwrap();
   let key = CString::new(key).unwrap(); 
   let statement = cass_statement_new(query.as_ptr(), 1);

  cass_statement_bind_string(statement, 0, key.as_ptr());

  let future = &mut*cass_session_execute(session, statement);
  cass_future_wait(future);

  let result:Result<(), CassError> = match cass_future_error_code(future) {
      CASS_OK => {
          let result = cass_future_get_result(future);
          let iterator = cass_iterator_from_result(result);

          if cass_iterator_next(iterator) > 0 {
              let row = cass_iterator_get_row(iterator);
              let value = cass_row_get_column(row, 0);
              let items_iterator = cass_iterator_from_collection(value);
              
              while cass_iterator_next(items_iterator) > 0 {
                  //const char* item;
                  let mut item = mem::zeroed();
                  let mut item_length = mem::zeroed();
                  //size_t item_length;
                  cass_value_get_string(cass_iterator_get_value(items_iterator), &mut item, &mut item_length);
                  println!("item: {:?}", item);
              }
              
                        };
                        Ok(())

      },
      rc => {
          print_error(future);
          Err(rc)
      }
  };
  


  cass_future_free(future);
  cass_statement_free(statement);

  result
}}

fn main() {unsafe{
  let cluster = &mut* create_cluster();
  let session = &mut* cass_session_new();

  let items:Vec<&str> = vec!("apple", "orange", "banana", "mango");

  cass_cluster_set_protocol_version(cluster, 2);

  match connect_session(session, cluster) {
      CASS_OK => (),
      _ => {
          cass_cluster_free(cluster);
          cass_session_free(session);
      }    
  }

  execute_query(session,
                "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \
                           'class': 'SimpleStrategy', 'replication_factor': '1' };").unwrap();

  execute_query(session,
                "CREATE TABLE IF NOT EXISTS examples.collections (key text, \
                                                    items set<text>, \
                                                    PRIMARY KEY (key))").unwrap();


  insert_into_collections(session, "test", items).unwrap();
  select_from_collections(session, "test").unwrap();

  let close_future = cass_session_close(session);
  cass_future_wait(close_future);
  cass_future_free(close_future);

  cass_cluster_free(cluster);
  cass_session_free(session);
}}
