//#![feature(plugin)]
//#![plugin(clippy)]
#![allow(float_cmp)]

extern crate cassandra_cpp_sys;
extern crate num;
extern crate threadpool;

use std::mem;
use std::ffi::CString;
use std::ffi::CStr;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;


use cassandra_cpp_sys::*;

const NUM_THREADS:usize = 1;
const NUM_IO_WORKER_THREADS:usize = 4;
const NUM_CONCURRENT_REQUESTS:usize = 10000;
const NUM_ITERATIONS:usize = 1000;

const DO_SELECTS:usize = 1;
const USE_PREPARED:usize = 1;

const big_strong:&'static str = 
"0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567\
0123456701234567012345670123456701234567012345670123456701234567";

//CassUuidGen* uuid_gen;

struct Status {
//  mutex:uv_mutex_t;
//  cond:uv_cond_t;
  count:i32
}

static status:Status = Status{count:0};

//fn status_init(status:&Status, initial_count:i32) -> isize {
//  let rc = uv_mutex_init(&status.mutex);
//  if rc != 0 {return rc}
//  rc = uv_cond_init(&status.cond);
//  if rc != 0 {return rc}
//  status.count = initial_count;
//  rc
//}

//fn status_destroy(status:&Status) {
//  uv_mutex_destroy(&status.mutex);
//  uv_cond_destroy(&status.cond);
//}
//
//fn status_notify(status: &Status) {
//  uv_mutex_lock(&status.mutex);
//  status.count-=1;
//  uv_cond_signal(&status.cond);
//  uv_mutex_unlock(&status.mutex);
//}
//
//fn status_wait(status:&Status, timeout_secs:u64) -> isize {
////  int count;
//  uv_mutex_lock(&status.mutex);
//  uv_cond_timedwait(&status.cond, &status.mutex, timeout_secs * 1000 * 1000 * 1000);
//  let count = status.count;
//  uv_mutex_unlock(&status.mutex);
//  return count;
//}

fn print_error(future:&mut CassFuture) {
    let message = mem::zeroed();
    let message_length = mem::zeroed();
  cass_future_error_message(future, &message, &message_length);
  println!("Error: {:?}",  raw2utf8(message,message_length));
}

fn create_cluster() -> CassCluster {
  let cluster = cass_cluster_new();
  cass_cluster_set_contact_points(cluster, "127.0.0.1");
  cass_cluster_set_credentials(cluster, "cassandra", "cassandra");
  cass_cluster_set_num_threads_io(cluster, NUM_IO_WORKER_THREADS);
  cass_cluster_set_queue_size_io(cluster, 10000);
  cass_cluster_set_pending_requests_low_water_mark(cluster, 5000);
  cass_cluster_set_pending_requests_high_water_mark(cluster, 10000);
  cass_cluster_set_core_connections_per_host(cluster, 1);
  cass_cluster_set_max_connections_per_host(cluster, 2);
  cass_cluster_set_max_requests_per_flush(cluster, 10000);
  return cluster;
}

fn connect_session(session: &mut CassSession, cluster: &mut CassCluster) -> CassError {
  let future = cass_session_connect_keyspace(session, cluster, "examples");

  cass_future_wait(future);
  let rc = cass_future_error_code(future);
  if (rc != CASS_OK) {
    print_error(future);
  }
  cass_future_free(future);

  return rc;
}

fn execute_query(session: &mut CassSession, query:&str) -> CassError {
  let statement = cass_statement_new(query, 0);

  let future = cass_session_execute(session, statement);
  cass_future_wait(future);

  let rc = cass_future_error_code(future);
  if (rc != CASS_OK) {
    print_error(future);
  }

  cass_future_free(future);
  cass_statement_free(statement);

  return rc;
}

fn prepare_query(session: &mut CassSession, query:&str, prepared:&CassPrepared) -> CassError {

  let future = cass_session_prepare(session, query);
  cass_future_wait(future);

  let rc = cass_future_error_code(future);
  if (rc != CASS_OK) {
    print_error(future);
  } else {
    let prepared = &cass_future_get_prepared(future);
  }

  cass_future_free(future);

  return rc;
}

fn compare_dbl(d1:f64, d2:f64) -> isize {
  if d1 < d2 {
    return -1;
  } else if d1 > d2 {
    return 1;
  } else {
    return 0;
  }
}

fn insert_into_perf(session: &mut CassSession, query:&str, prepared:Option<CassPrepared>) {
  let futures:Vec<CassFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS);

  let collection = cass_collection_new(CASS_COLLECTION_TYPE_SET, 2);
  cass_collection_append_string(collection, "jazz");
  cass_collection_append_string(collection, "2013");
    let statement = mem::zeroed();
    
  for i in 0..NUM_CONCURRENT_REQUESTS {
      match prepared {
          Some(prepared) => cass_prepared_bind(prepared),
          None => cass_statement_new(query, 5)
      }
      let id = mem::zeroed();
      let big_string = mem::zeroed();
    let uuid_gen = mem::zeroed();
    cass_uuid_gen_time(uuid_gen, &id);
    cass_statement_bind_uuid(statement, 0, id);
    cass_statement_bind_string(statement, 1, big_string);
    cass_statement_bind_string(statement, 2, big_string);
    cass_statement_bind_string(statement, 3, big_string);
    cass_statement_bind_collection(statement, 4, collection);

    futures[i] = cass_session_execute(session, statement);

    cass_statement_free(statement);
  }

  for i in 0..NUM_CONCURRENT_REQUESTS {
    let future = futures[i];
    let rc = cass_future_error_code(future);
    if rc != CASS_OK {
      print_error(future);
    }
    cass_future_free(future);
  }

  cass_collection_free(collection);
}

fn run_insert_queries(session:&mut CassSession) {

  let insert_prepared:Option<CassPrepared> = None;
  let insert_query = "INSERT INTO songs (id, title, album, artist, tags) VALUES (?, ?, ?, ?, ?);".to_owned();

  if prepare_query(session, insert_query, &insert_prepared) == CASS_OK {
    for i in 0..NUM_ITERATIONS {
      insert_into_perf(session, insert_query, insert_prepared);
    }
    cass_prepared_free(insert_prepared);
  }

//  status_notify(&status);
}

fn select_from_perf(session: &mut CassSession, query:&str, prepared:&CassPrepared) {
let futures:Vec<CassFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS);

  for i in 0..NUM_CONCURRENT_REQUESTS {
let statement:Option<CassStatement> = None;
    match prepared {
        Some(prepared) => cass_prepared_bind(prepared),
        None => statement = cass_statement_new(query, 0)
    }
    
    futures[i] = cass_session_execute(session, statement);

    cass_statement_free(statement);
  }

  for i in 0..NUM_CONCURRENT_REQUESTS {
    let future = futures[i];
    let rc = cass_future_error_code(future);
    if (rc != CASS_OK) {
      print_error(future);
    } else {
      let result = cass_future_get_result(future);
      assert!(cass_result_column_count(result) == 6);
      cass_result_free(result);
    }
    cass_future_free(future);
  }
}

fn run_select_queries(session:&mut CassSession) {
  let select_query = "SELECT * FROM songs WHERE id = a98d21b2-1900-11e4-b97b-e5e358e71e0d".to_owned();
  let select_prepared = mem::zeroed();
  
  if prepare_query(session, select_query, &select_prepared) == CASS_OK {
    for i in 0..NUM_ITERATIONS {
      select_from_perf(session, select_query, select_prepared);
    }
    cass_prepared_free(select_prepared);
  }

  status_notify(&status);
}

fn main() {
//  uv_thread_t threads[NUM_THREADS];
//  CassCluster* cluster = NULL;
//  CassSession* session = NULL;
//  CassFuture* close_future = NULL;

//  status_init(&status, NUM_THREADS);

  cass_log_set_level(CASS_LOG_INFO);

  let cluster = create_cluster();
  let uuid_gen = cass_uuid_gen_new();
  let session = cass_session_new();

  if (connect_session(session, cluster) != CASS_OK) {
    cass_cluster_free(cluster);
    cass_session_free(session);
    return -1;
  }

  execute_query(session,
"INSERT INTO songs (id, title, album, artist, tags) VALUES \
(a98d21b2-1900-11e4-b97b-e5e358e71e0d, \
'La Petite Tonkinoise', 'Bye Bye Blackbird', 'Joséphine Baker', { 'jazz', '2013' });");


  for i in 0..NUM_THREADS {
//    uv_thread_create(&threads[i], run_select_queries, (void*)session);
  }

  while (status_wait(&status, 5) > 0) {
      let metrics = mem::zeroed();
    cass_session_get_metrics(session, &metrics);
    println!("rate stats (requests/second): mean {:?} 1m {:?} 5m {:?} 10m {:?}",
           metrics.requests.mean_rate,
           metrics.requests.one_minute_rate,
           metrics.requests.five_minute_rate,
           metrics.requests.fifteen_minute_rate);
  }

    let metrics = mem::zeroed();
  cass_session_get_metrics(session, &metrics);
  println!("final stats (microseconds): min {:?} max {:?} median {:?} 75th {:?} 95th {:?} 98th {:?} 99th {:?} 99.9th {:?}",
         metrics.requests.min, metrics.requests.max,
         metrics.requests.median, metrics.requests.percentile_75th,
         metrics.requests.percentile_95th, metrics.requests.percentile_98th,
         metrics.requests.percentile_99th, metrics.requests.percentile_999th);

  for i in 0..NUM_THREADS {
    //uv_thread_join(&threads[i]);
  }

  let close_future = cass_session_close(session);
  cass_future_wait(close_future);
  cass_future_free(close_future);
  cass_cluster_free(cluster);
  cass_uuid_gen_free(uuid_gen);


  return 0;
}
