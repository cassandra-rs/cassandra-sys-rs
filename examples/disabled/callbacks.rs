//uv_mutex_t mutex;
//uv_cond_t cond;
//CassFuture* close_future = NULL;
//CassUuidGen* uuid_gen = NULL;

//fn wait_exit() {
//  uv_mutex_lock(&mutex);
//  while (close_future == NULL) {
//    uv_cond_wait(&cond, &mutex);
//  }
//  uv_mutex_unlock(&mutex);
//  cass_future_wait(close_future);
//  cass_future_free(close_future);
//}
//
//void signal_exit(CassSession* session) {
//  uv_mutex_lock(&mutex);
//  close_future = cass_session_close(session);
//  uv_cond_signal(&cond);
//  uv_mutex_unlock(&mutex);
//}

//void on_create_keyspace(CassFuture* future, void* data);
//void on_create_table(CassFuture* future, void* data);
//
//void on_insert(CassFuture* future, void* data);
//void on_select(CassFuture* future, void* data);
//
//void on_session_connect(CassFuture* future, void* data);
//void on_session_close(CassFuture* future, void* data);

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

fn execute_query(session: &mut CassSession, query: &str) -> Result<(), CassError> {
    unsafe {
        let query = CString::new(query).unwrap();
        let statement = cass_statement_new(query.as_ptr(), 0);
        let future = cass_session_execute(session, statement);
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


fn on_session_connect(future:CassFuture, data: *mut c_void) {
  let session = data as *mut CassSession;
  let code = cass_future_error_code(future);

  match code {
      CASS_OK => {},
      rc => {
          
      }
    print_error(future);
    uv_cond_signal(&cond);
    return;
  }

  execute_query(session,
                "CREATE KEYSPACE examples WITH replication = { "
                "'class': 'SimpleStrategy', 'replication_factor': '3' };",
                on_create_keyspace);
}

void on_create_keyspace(CassFuture* future, void* data) {
  CassError code = cass_future_error_code(future);
  if (code != CASS_OK) {
    print_error(future);
  }

  execute_query((CassSession*)data,
                "CREATE TABLE callbacks "
                "(key timeuuid PRIMARY KEY, value bigint)",
                on_create_table);
}

void on_create_table(CassFuture* future, void* data) {
  const char* insert_query = "INSERT INTO callbacks (key, value) "
                             "VALUES (?, ?)";
  CassUuid key;
  CassStatement* statement = NULL;
  CassFuture* insert_future = NULL;

  CassError code = cass_future_error_code(future);
  if (code != CASS_OK) {
    print_error(future);
  }

  statement = cass_statement_new(insert_query, 2);

  cass_uuid_gen_time(uuid_gen, &key);
  cass_statement_bind_uuid(statement, 0, key);
  cass_statement_bind_int64(statement, 1, cass_uuid_timestamp(key));

  insert_future = cass_session_execute((CassSession*)data, statement);

  cass_future_set_callback(insert_future, on_insert, data);

  cass_statement_free(statement);
  cass_future_free(insert_future);
}

void on_insert(CassFuture* future, void* data) {
  CassError code = cass_future_error_code(future);
  if (code != CASS_OK) {
    print_error(future);
    signal_exit((CassSession*)data);
  } else {
    const char* select_query = "SELECT * FROM callbacks";
    CassStatement* statement
        = cass_statement_new(select_query, 0);
    CassFuture* select_future
        = cass_session_execute((CassSession*)data, statement);

    cass_future_set_callback(select_future, on_select, data);

    cass_statement_free(statement);
    cass_future_free(select_future);
  }
}

void on_select(CassFuture* future, void* data) {
  CassError code = cass_future_error_code(future);
  if (code != CASS_OK) {
    print_error(future);
  } else {
    const CassResult* result = cass_future_get_result(future);
    CassIterator* iterator = cass_iterator_from_result(result);
    while (cass_iterator_next(iterator)) {
      CassUuid key;
      char key_str[CASS_UUID_STRING_LENGTH];
      cass_uint64_t value = 0;
      const CassRow* row = cass_iterator_get_row(iterator);

      cass_value_get_uuid(cass_row_get_column(row, 0), &key);

      cass_uuid_string(key, key_str);
      cass_value_get_int64(cass_row_get_column(row, 1), (cass_int64_t*)&value);

      printf("%s, %llu\n", key_str, (unsigned long long)value);
    }
    cass_iterator_free(iterator);
    cass_result_free(result);
  }

  signal_exit((CassSession*)data);
}

fn main() {
  CassCluster* cluster = create_cluster();
  CassSession* session = cass_session_new();

  uuid_gen = cass_uuid_gen_new();

  uv_mutex_init(&mutex);
  uv_cond_init(&cond);

  connect_session(session, cluster, on_session_connect);

  /* Code running in parallel with queries */

  wait_exit();

  uv_cond_destroy(&cond);
  uv_mutex_destroy(&mutex);

  cass_cluster_free(cluster);
  cass_uuid_gen_free(uuid_gen);
  cass_session_free(session);

  return 0;
}