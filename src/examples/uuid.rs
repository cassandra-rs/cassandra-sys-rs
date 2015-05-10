extern crate cql_bindgen;   

use std::mem;
use std::ffi::CStr;
use std::str;

use cql_bindgen::*;

const CASS_UUID_STRING_LENGTH:usize = 37;


fn create_cluster() -> &'static mut CassCluster {unsafe{
    let cluster = cass_cluster_new();
    cass_cluster_set_contact_points(cluster,str2ref("127.0.0.1"));
    &mut*cluster 
}}

fn connect_session(session:&mut CassSession, cluster:&CassCluster) -> CassError {unsafe{
    let future = cass_session_connect(session,cluster);
    cass_future_wait(future);
    let err = cass_future_error_code(future);
    cass_future_free(future);
    err
}}

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {unsafe{
    println!("{:?}",query);
    let statement = cass_statement_new(str2ref(query), 0);
    let future = cass_session_execute(session,statement);
    cass_future_wait(future);
    let _ = cass_future_error_code(future);
    let rc = cass_future_error_code(future);
    cass_future_free(future);
    match rc {
      CASS_OK => Ok(()),
      err => Err(err)
  }
}}

fn insert_into_log(session:&mut CassSession, key:&str, time:CassUuid,entry:&str) -> Result<(),CassError> {unsafe{
  let query = str2ref("INSERT INTO examples.log (key, time, entry) VALUES (?, ?, ?);");

  let statement:*mut CassStatement = cass_statement_new(query, 3);

  cass_statement_bind_string(statement, 0, str2ref(key));
  cass_statement_bind_uuid(statement, 1, time);
  cass_statement_bind_string(statement, 2, str2ref(entry));

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
          println!("{:?}",raw2utf8(message,message_length));
          Err(err)
      }    
  }
}}

fn select_from_log(session:&mut CassSession, key:&str) -> Result<(),CassError> {unsafe{
  let query = str2ref("SELECT * FROM examples.log WHERE key = ?");

  let statement = cass_statement_new(query, 1);

  cass_statement_bind_string(statement, 0, str2ref(key));

  let future = cass_session_execute(session, statement);
  cass_future_wait(future);

  let rc = cass_future_error_code(future);
  if rc != CASS_OK {
    //print_error(future);
  } else {
    let result = cass_future_get_result(future);
    let iterator = cass_iterator_from_result(result);
    

    while cass_iterator_next(iterator) > 0 {
      let row = cass_iterator_get_row(iterator);
      //const char* key;
      let mut key_length:size_t = mem::zeroed();
      let mut time:Struct_CassUuid_ = mem::zeroed();
      let mut entry = mem::zeroed();
      let mut entry_length = mem::zeroed();
      let mut time_str:[i8;CASS_UUID_STRING_LENGTH] = [0;CASS_UUID_STRING_LENGTH];

      cass_value_get_string(cass_row_get_column(row, 0), &mut str2ref(key), &mut key_length);
      cass_value_get_uuid(cass_row_get_column(row, 1), &mut time);
      cass_value_get_string(cass_row_get_column(row, 2), &mut entry, &mut entry_length);
      let mut output:i8 = mem::zeroed();
      cass_uuid_string(time,&mut output);
      let output = CStr::from_ptr(&output);
      println!("{:?}", str::from_utf8(output.to_bytes()).unwrap());

      cass_uuid_string(time, time_str[..].as_mut_ptr());

    //  println!("{:?} {:?} {:?}", raw2utf8(key_length, key), time_str, raw2utf8(entry_length, entry));
    }

    cass_result_free(result);
    cass_iterator_free(iterator);
  }

  cass_future_free(future);
  cass_statement_free(statement);

  match rc {
      CASS_OK => Ok(()),
      err => Err(err)
  }
}}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let uuid_gen = cass_uuid_gen_new();
    let mut uuid = mem::zeroed();

    match connect_session(&mut*session, cluster) {
        CASS_OK => {
//            let output = &mut Basic{bln:0,flt:0f32,dbl:0f64,i32:0,i64:0};
            execute_query(&mut*session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \
                           'class': 'SimpleStrategy', 'replication_factor': '1' };").unwrap();
            execute_query(&mut*session, "CREATE TABLE IF NOT EXISTS examples.log (key text, time timeuuid, entry text, \
                                              PRIMARY KEY (key, time));").unwrap();
            
             cass_uuid_gen_time(uuid_gen, &mut uuid);

   insert_into_log(&mut*session, "test", uuid, "Log entry #1").unwrap();

  cass_uuid_gen_time(uuid_gen, &mut uuid);
  insert_into_log(&mut*session, "test", uuid, "Log entry #2").unwrap();

  cass_uuid_gen_time(uuid_gen, &mut uuid);
  insert_into_log(&mut*session, "test", uuid, "Log entry #3").unwrap();

  cass_uuid_gen_time(uuid_gen, &mut uuid);
  insert_into_log(&mut*session, "test", uuid, "Log entry #4").unwrap();

  select_from_log(&mut*session, "test").unwrap();

            let close_future = cass_session_close(session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        },
        _ => {}
    }
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
