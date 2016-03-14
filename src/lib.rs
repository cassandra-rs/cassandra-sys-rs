#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature="clippy", feature(plugin))]
// #![cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use]
extern crate log;
extern crate libc;

pub use ffi_util::*;

pub use cassandra::cass_bool_t;
pub use cassandra::Enum_Unnamed1::{cass_true, cass_false};

pub use cassandra::CassError;
pub use cassandra::Enum_CassError_::{CASS_OK, 
    CASS_ERROR_LIB_BAD_PARAMS,
    CASS_ERROR_LIB_NO_STREAMS,
    CASS_ERROR_LIB_UNABLE_TO_INIT,
    CASS_ERROR_LIB_MESSAGE_ENCODE,
    CASS_ERROR_LIB_HOST_RESOLUTION,
    CASS_ERROR_LIB_UNEXPECTED_RESPONSE,
    CASS_ERROR_LIB_REQUEST_QUEUE_FULL,
    CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD,
    CASS_ERROR_LIB_WRITE_ERROR,
    CASS_ERROR_LIB_NO_HOSTS_AVAILABLE,
    CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS,
    CASS_ERROR_LIB_INVALID_ITEM_COUNT,
    CASS_ERROR_LIB_INVALID_VALUE_TYPE,
    CASS_ERROR_LIB_REQUEST_TIMED_OUT,
    CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE,
    CASS_ERROR_LIB_CALLBACK_ALREADY_SET,
    CASS_ERROR_LIB_INVALID_STATEMENT_TYPE,
    CASS_ERROR_LIB_NAME_DOES_NOT_EXIST,
    CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL,
    CASS_ERROR_LIB_NULL_VALUE,
    CASS_ERROR_LIB_NOT_IMPLEMENTED,
    CASS_ERROR_LIB_UNABLE_TO_CONNECT,
    CASS_ERROR_LIB_UNABLE_TO_CLOSE,
    CASS_ERROR_LIB_NO_PAGING_STATE,
    CASS_ERROR_LIB_PARAMETER_UNSET,
    CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE,
    CASS_ERROR_LIB_INVALID_FUTURE_TYPE,
    CASS_ERROR_SERVER_SERVER_ERROR,
    CASS_ERROR_SERVER_PROTOCOL_ERROR,
    CASS_ERROR_SERVER_BAD_CREDENTIALS,
    CASS_ERROR_SERVER_UNAVAILABLE,
    CASS_ERROR_SERVER_OVERLOADED,
    CASS_ERROR_SERVER_IS_BOOTSTRAPPING,
    CASS_ERROR_SERVER_TRUNCATE_ERROR,
    CASS_ERROR_SERVER_WRITE_TIMEOUT,
    CASS_ERROR_SERVER_READ_TIMEOUT,
    CASS_ERROR_SERVER_READ_FAILURE,
    CASS_ERROR_SERVER_FUNCTION_FAILURE,
    CASS_ERROR_SERVER_WRITE_FAILURE,
    CASS_ERROR_SERVER_SYNTAX_ERROR,
    CASS_ERROR_SERVER_UNAUTHORIZED,
    CASS_ERROR_SERVER_INVALID_QUERY,
    CASS_ERROR_SERVER_CONFIG_ERROR,
    CASS_ERROR_SERVER_ALREADY_EXISTS,
    CASS_ERROR_SERVER_UNPREPARED,
    CASS_ERROR_SSL_INVALID_CERT,
    CASS_ERROR_SSL_INVALID_PRIVATE_KEY,
    CASS_ERROR_SSL_NO_PEER_CERT,
    CASS_ERROR_SSL_INVALID_PEER_CERT,
    CASS_ERROR_SSL_IDENTITY_MISMATCH,
    CASS_ERROR_SSL_PROTOCOL_ERROR,
    CASS_ERROR_LAST_ENTRY,

};

pub use cassandra::Enum_CassSslVerifyFlags::{CASS_SSL_VERIFY_NONE, CASS_SSL_VERIFY_PEER_CERT,
                                             CASS_SSL_VERIFY_PEER_IDENTITY};

pub use cassandra::Enum_CassBatchType_::CASS_BATCH_TYPE_LOGGED;

pub use cassandra::Enum_CassLogLevel_::CASS_LOG_INFO;


pub use cassandra::Enum_CassValueType_::{CASS_VALUE_TYPE_UNKNOWN, CASS_VALUE_TYPE_CUSTOM, CASS_VALUE_TYPE_ASCII,
                                         CASS_VALUE_TYPE_BIGINT, CASS_VALUE_TYPE_BLOB, CASS_VALUE_TYPE_BOOLEAN,
                                         CASS_VALUE_TYPE_COUNTER, CASS_VALUE_TYPE_DECIMAL, CASS_VALUE_TYPE_DOUBLE,
                                         CASS_VALUE_TYPE_FLOAT, CASS_VALUE_TYPE_INT, CASS_VALUE_TYPE_TEXT,
                                         CASS_VALUE_TYPE_TIMESTAMP, CASS_VALUE_TYPE_UUID, CASS_VALUE_TYPE_VARCHAR,
                                         CASS_VALUE_TYPE_VARINT, CASS_VALUE_TYPE_TIMEUUID, CASS_VALUE_TYPE_INET,
                                         CASS_VALUE_TYPE_DATE, CASS_VALUE_TYPE_TIME, CASS_VALUE_TYPE_SMALL_INT,
                                         CASS_VALUE_TYPE_TINY_INT, CASS_VALUE_TYPE_LIST, CASS_VALUE_TYPE_MAP,
                                         CASS_VALUE_TYPE_SET, CASS_VALUE_TYPE_UDT, CASS_VALUE_TYPE_TUPLE,
                                         CASS_VALUE_TYPE_LAST_ENTRY};

pub use cassandra::Enum_CassCollectionType_::{CASS_COLLECTION_TYPE_SET, CASS_COLLECTION_TYPE_LIST,
                                              CASS_COLLECTION_TYPE_MAP};

// pub use cassandra::ffi_util::raw2utf8;
pub use cassandra::CassInet;
pub use cassandra::CassUuid;
pub use cassandra::CassCluster;

pub use cassandra::CassSession;
pub use cassandra::CassStatement;
pub use cassandra::CassBatch;
pub use cassandra::CassFuture;
pub use cassandra::CassPrepared;
pub use cassandra::CassResult;
pub use cassandra::CassErrorResult;
pub use cassandra::CassIterator;
pub use cassandra::CassRow;

pub use cassandra::CassValue;
pub use cassandra::CassDataType;
pub use cassandra::CassFunctionMeta;
pub use cassandra::CassAggregateMeta;
pub use cassandra::CassCollection;
pub use cassandra::CassTuple;
pub use cassandra::CassUserType;
pub use cassandra::CassSsl;
pub use cassandra::CassSchemaMeta;
pub use cassandra::CassKeyspaceMeta;
pub use cassandra::CassTableMeta;
pub use cassandra::CassColumnMeta;
pub use cassandra::CassUuidGen;
pub use cassandra::CassTimestampGen;
pub use cassandra::CassRetryPolicy;
pub use cassandra::CassCustomPayload;
pub use cassandra::CassMetrics;
pub use cassandra::CassConsistency;
pub use cassandra::CassWriteType;
pub use cassandra::CassValueType;
pub use cassandra::CassCollectionType;
pub use cassandra::CassBatchType;
pub use cassandra::CassIteratorType;
pub use cassandra::CassLogLevel;
pub use cassandra::CassSslVerifyFlags;
pub use cassandra::CassColumnType;
pub use cassandra::CassErrorSource;
pub use cassandra::CassLogMessage;
pub use cassandra::CassLogCallback;
pub use cassandra::CassFutureCallback;

pub use cassandra::cass_cluster_new;
pub use cassandra::cass_cluster_free;
pub use cassandra::cass_cluster_set_contact_points;
pub use cassandra::cass_cluster_set_contact_points_n;
pub use cassandra::cass_cluster_set_port;
pub use cassandra::cass_cluster_set_ssl;
pub use cassandra::cass_cluster_set_protocol_version;
pub use cassandra::cass_cluster_set_num_threads_io;
pub use cassandra::cass_cluster_set_queue_size_io;
pub use cassandra::cass_cluster_set_queue_size_event;
pub use cassandra::cass_cluster_set_queue_size_log;
pub use cassandra::cass_cluster_set_core_connections_per_host;
pub use cassandra::cass_cluster_set_max_connections_per_host;
pub use cassandra::cass_cluster_set_reconnect_wait_time;
pub use cassandra::cass_cluster_set_max_concurrent_creation;
pub use cassandra::cass_cluster_set_max_concurrent_requests_threshold;
pub use cassandra::cass_cluster_set_max_requests_per_flush;
pub use cassandra::cass_cluster_set_write_bytes_high_water_mark;
pub use cassandra::cass_cluster_set_write_bytes_low_water_mark;
pub use cassandra::cass_cluster_set_pending_requests_high_water_mark;
pub use cassandra::cass_cluster_set_pending_requests_low_water_mark;
pub use cassandra::cass_cluster_set_connect_timeout;


pub use cassandra::cass_cluster_set_request_timeout;
pub use cassandra::cass_cluster_set_credentials;
pub use cassandra::cass_cluster_set_credentials_n;
pub use cassandra::cass_cluster_set_load_balance_round_robin;
pub use cassandra::cass_cluster_set_load_balance_dc_aware;
pub use cassandra::cass_cluster_set_load_balance_dc_aware_n;
pub use cassandra::cass_cluster_set_token_aware_routing;
pub use cassandra::cass_cluster_set_latency_aware_routing;
pub use cassandra::cass_cluster_set_latency_aware_routing_settings;
pub use cassandra::cass_cluster_set_whitelist_filtering;
pub use cassandra::cass_cluster_set_whitelist_filtering_n;
pub use cassandra::cass_cluster_set_tcp_nodelay;
pub use cassandra::cass_cluster_set_tcp_keepalive;
pub use cassandra::cass_cluster_set_timestamp_gen;
pub use cassandra::cass_cluster_set_connection_heartbeat_interval;
pub use cassandra::cass_cluster_set_connection_idle_timeout;
pub use cassandra::cass_cluster_set_retry_policy;
pub use cassandra::cass_cluster_set_use_schema;
pub use cassandra::cass_session_new;
pub use cassandra::cass_session_free;
pub use cassandra::cass_session_connect;
pub use cassandra::cass_session_connect_keyspace;
pub use cassandra::cass_session_connect_keyspace_n;
pub use cassandra::cass_session_close;
pub use cassandra::cass_session_prepare;
pub use cassandra::cass_session_prepare_n;
pub use cassandra::cass_session_execute;
pub use cassandra::cass_session_execute_batch;
pub use cassandra::cass_session_get_schema_meta;
pub use cassandra::cass_session_get_metrics;
pub use cassandra::cass_schema_meta_free;
pub use cassandra::cass_schema_meta_snapshot_version;
pub use cassandra::cass_schema_meta_keyspace_by_name;
pub use cassandra::cass_schema_meta_keyspace_by_name_n;
pub use cassandra::cass_keyspace_meta_table_by_name;
pub use cassandra::cass_keyspace_meta_table_by_name_n;
pub use cassandra::cass_keyspace_meta_user_type_by_name;
pub use cassandra::cass_keyspace_meta_user_type_by_name_n;
pub use cassandra::cass_keyspace_meta_function_by_name;
pub use cassandra::cass_keyspace_meta_function_by_name_n;
pub use cassandra::cass_keyspace_meta_aggregate_by_name;
pub use cassandra::cass_keyspace_meta_aggregate_by_name_n;
pub use cassandra::cass_keyspace_meta_name;
pub use cassandra::cass_keyspace_meta_field_by_name;
pub use cassandra::cass_keyspace_meta_field_by_name_n;
pub use cassandra::cass_table_meta_column_by_name;
pub use cassandra::cass_table_meta_column_by_name_n;
pub use cassandra::cass_table_meta_name;
pub use cassandra::cass_table_meta_column_count;
pub use cassandra::cass_table_meta_column;
pub use cassandra::cass_table_meta_partition_key_count;
pub use cassandra::cass_table_meta_partition_key;
pub use cassandra::cass_table_meta_clustering_key_count;
pub use cassandra::cass_table_meta_clustering_key;
pub use cassandra::cass_table_meta_field_by_name;
pub use cassandra::cass_table_meta_field_by_name_n;
pub use cassandra::cass_column_meta_name;
pub use cassandra::cass_column_meta_type;
pub use cassandra::cass_column_meta_data_type;
pub use cassandra::cass_column_meta_field_by_name;
pub use cassandra::cass_column_meta_field_by_name_n;
pub use cassandra::cass_function_meta_name;
pub use cassandra::cass_function_meta_full_name;
pub use cassandra::cass_function_meta_body;
pub use cassandra::cass_function_meta_language;
pub use cassandra::cass_function_meta_called_on_null_input;
pub use cassandra::cass_function_meta_argument_count;
pub use cassandra::cass_function_meta_argument;
pub use cassandra::cass_function_meta_argument_type_by_name;
pub use cassandra::cass_function_meta_argument_type_by_name_n;
pub use cassandra::cass_function_meta_return_type;
pub use cassandra::cass_function_meta_field_by_name;
pub use cassandra::cass_function_meta_field_by_name_n;
pub use cassandra::cass_aggregate_meta_name;
pub use cassandra::cass_aggregate_meta_full_name;
pub use cassandra::cass_aggregate_meta_argument_count;
pub use cassandra::cass_aggregate_meta_argument_type;
pub use cassandra::cass_aggregate_meta_return_type;
pub use cassandra::cass_aggregate_meta_state_type;
pub use cassandra::cass_aggregate_meta_state_func;
pub use cassandra::cass_aggregate_meta_final_func;
pub use cassandra::cass_aggregate_meta_init_cond;
pub use cassandra::cass_aggregate_meta_field_by_name;
pub use cassandra::cass_aggregate_meta_field_by_name_n;
pub use cassandra::cass_ssl_new;
pub use cassandra::cass_ssl_free;
pub use cassandra::cass_ssl_add_trusted_cert;
pub use cassandra::cass_ssl_add_trusted_cert_n;
pub use cassandra::cass_ssl_set_verify_flags;
pub use cassandra::cass_ssl_set_cert;
pub use cassandra::cass_ssl_set_cert_n;
pub use cassandra::cass_ssl_set_private_key;
pub use cassandra::cass_ssl_set_private_key_n;
pub use cassandra::cass_future_free;
pub use cassandra::cass_future_set_callback;
pub use cassandra::cass_future_ready;
pub use cassandra::cass_future_wait;
pub use cassandra::cass_future_wait_timed;
pub use cassandra::cass_future_get_result;
pub use cassandra::cass_future_get_error_result;
pub use cassandra::cass_future_get_prepared;
pub use cassandra::cass_future_error_code;
pub use cassandra::cass_future_error_message;
pub use cassandra::cass_future_custom_payload_item_count;
pub use cassandra::cass_future_custom_payload_item;
pub use cassandra::cass_statement_new;
pub use cassandra::cass_statement_new_n;
pub use cassandra::cass_statement_free;
pub use cassandra::cass_statement_add_key_index;
pub use cassandra::cass_statement_set_keyspace;
pub use cassandra::cass_statement_set_keyspace_n;
pub use cassandra::cass_statement_set_consistency;
pub use cassandra::cass_statement_set_serial_consistency;
pub use cassandra::cass_statement_set_paging_size;
pub use cassandra::cass_statement_set_paging_state;
pub use cassandra::cass_statement_set_paging_state_token;
pub use cassandra::cass_statement_set_timestamp;
pub use cassandra::cass_statement_set_retry_policy;
pub use cassandra::cass_statement_set_custom_payload;
pub use cassandra::cass_statement_bind_null;
pub use cassandra::cass_statement_bind_null_by_name;
pub use cassandra::cass_statement_bind_null_by_name_n;
pub use cassandra::cass_statement_bind_int8;
pub use cassandra::cass_statement_bind_int8_by_name;
pub use cassandra::cass_statement_bind_int8_by_name_n;
pub use cassandra::cass_statement_bind_int16;
pub use cassandra::cass_statement_bind_int16_by_name;
pub use cassandra::cass_statement_bind_int16_by_name_n;
pub use cassandra::cass_statement_bind_int32;
pub use cassandra::cass_statement_bind_int32_by_name;
pub use cassandra::cass_statement_bind_int32_by_name_n;
pub use cassandra::cass_statement_bind_uint32;
pub use cassandra::cass_statement_bind_uint32_by_name;
pub use cassandra::cass_statement_bind_uint32_by_name_n;
pub use cassandra::cass_statement_bind_int64;
pub use cassandra::cass_statement_bind_int64_by_name;
pub use cassandra::cass_statement_bind_int64_by_name_n;
pub use cassandra::cass_statement_bind_float;
pub use cassandra::cass_statement_bind_float_by_name;
pub use cassandra::cass_statement_bind_float_by_name_n;
pub use cassandra::cass_statement_bind_double;
pub use cassandra::cass_statement_bind_double_by_name;
pub use cassandra::cass_statement_bind_double_by_name_n;
pub use cassandra::cass_statement_bind_bool;
pub use cassandra::cass_statement_bind_bool_by_name;
pub use cassandra::cass_statement_bind_bool_by_name_n;
pub use cassandra::cass_statement_bind_string;
pub use cassandra::cass_statement_bind_string_n;
pub use cassandra::cass_statement_bind_string_by_name;
pub use cassandra::cass_statement_bind_string_by_name_n;
pub use cassandra::cass_statement_bind_bytes;
pub use cassandra::cass_statement_bind_bytes_by_name;
pub use cassandra::cass_statement_bind_bytes_by_name_n;
pub use cassandra::cass_statement_bind_uuid;
pub use cassandra::cass_statement_bind_uuid_by_name;
pub use cassandra::cass_statement_bind_uuid_by_name_n;
pub use cassandra::cass_statement_bind_inet;
pub use cassandra::cass_statement_bind_inet_by_name;
pub use cassandra::cass_statement_bind_inet_by_name_n;
pub use cassandra::cass_statement_bind_decimal;
pub use cassandra::cass_statement_bind_decimal_by_name;
pub use cassandra::cass_statement_bind_decimal_by_name_n;
pub use cassandra::cass_statement_bind_collection;
pub use cassandra::cass_statement_bind_collection_by_name;
pub use cassandra::cass_statement_bind_collection_by_name_n;
pub use cassandra::cass_statement_bind_tuple;
pub use cassandra::cass_statement_bind_tuple_by_name;
pub use cassandra::cass_statement_bind_tuple_by_name_n;
pub use cassandra::cass_statement_bind_user_type;
pub use cassandra::cass_statement_bind_user_type_by_name;
pub use cassandra::cass_statement_bind_user_type_by_name_n;
pub use cassandra::cass_prepared_free;
pub use cassandra::cass_prepared_bind;
pub use cassandra::cass_prepared_parameter_name;
pub use cassandra::cass_prepared_parameter_data_type;
pub use cassandra::cass_prepared_parameter_data_type_by_name;
pub use cassandra::cass_prepared_parameter_data_type_by_name_n;
pub use cassandra::cass_batch_new;
pub use cassandra::cass_batch_free;
pub use cassandra::cass_batch_set_consistency;
pub use cassandra::cass_batch_set_serial_consistency;
pub use cassandra::cass_batch_set_timestamp;
pub use cassandra::cass_batch_set_retry_policy;
pub use cassandra::cass_batch_set_custom_payload;
pub use cassandra::cass_batch_add_statement;
pub use cassandra::cass_data_type_new;
pub use cassandra::cass_data_type_new_from_existing;
pub use cassandra::cass_data_type_new_tuple;
pub use cassandra::cass_data_type_new_udt;
pub use cassandra::cass_data_type_free;
pub use cassandra::cass_data_type_type;
pub use cassandra::cass_data_type_type_name;
pub use cassandra::cass_data_type_set_type_name;
pub use cassandra::cass_data_type_set_type_name_n;
pub use cassandra::cass_data_type_keyspace;
pub use cassandra::cass_data_type_set_keyspace;
pub use cassandra::cass_data_type_set_keyspace_n;
pub use cassandra::cass_data_type_class_name;
pub use cassandra::cass_data_type_set_class_name;
pub use cassandra::cass_data_type_set_class_name_n;
pub use cassandra::cass_data_type_sub_type_count;
pub use cassandra::cass_data_sub_type_count;
pub use cassandra::cass_data_type_sub_data_type;
pub use cassandra::cass_data_type_sub_data_type_by_name;
pub use cassandra::cass_data_type_sub_data_type_by_name_n;
pub use cassandra::cass_data_type_sub_type_name;
pub use cassandra::cass_data_type_add_sub_type;
pub use cassandra::cass_data_type_add_sub_type_by_name;
pub use cassandra::cass_data_type_add_sub_type_by_name_n;
pub use cassandra::cass_data_type_add_sub_value_type;
pub use cassandra::cass_data_type_add_sub_value_type_by_name;
pub use cassandra::cass_data_type_add_sub_value_type_by_name_n;
pub use cassandra::cass_collection_new;
pub use cassandra::cass_collection_new_from_data_type;
pub use cassandra::cass_collection_free;
pub use cassandra::cass_collection_data_type;
pub use cassandra::cass_collection_append_int8;
pub use cassandra::cass_collection_append_int16;
pub use cassandra::cass_collection_append_int32;
pub use cassandra::cass_collection_append_uint32;
pub use cassandra::cass_collection_append_int64;
pub use cassandra::cass_collection_append_float;
pub use cassandra::cass_collection_append_double;
pub use cassandra::cass_collection_append_bool;
pub use cassandra::cass_collection_append_string;
pub use cassandra::cass_collection_append_string_n;
pub use cassandra::cass_collection_append_bytes;
pub use cassandra::cass_collection_append_uuid;
pub use cassandra::cass_collection_append_inet;
pub use cassandra::cass_collection_append_decimal;
pub use cassandra::cass_collection_append_collection;
pub use cassandra::cass_collection_append_tuple;
pub use cassandra::cass_collection_append_user_type;
pub use cassandra::cass_tuple_new;
pub use cassandra::cass_tuple_new_from_data_type;
pub use cassandra::cass_tuple_free;
pub use cassandra::cass_tuple_data_type;
pub use cassandra::cass_tuple_set_null;
pub use cassandra::cass_tuple_set_int8;
pub use cassandra::cass_tuple_set_int16;
pub use cassandra::cass_tuple_set_int32;
pub use cassandra::cass_tuple_set_uint32;
pub use cassandra::cass_tuple_set_int64;
pub use cassandra::cass_tuple_set_float;
pub use cassandra::cass_tuple_set_double;
pub use cassandra::cass_tuple_set_bool;
pub use cassandra::cass_tuple_set_string;
pub use cassandra::cass_tuple_set_string_n;
pub use cassandra::cass_tuple_set_bytes;
pub use cassandra::cass_tuple_set_uuid;
pub use cassandra::cass_tuple_set_inet;
pub use cassandra::cass_tuple_set_decimal;
pub use cassandra::cass_tuple_set_collection;
pub use cassandra::cass_tuple_set_tuple;
pub use cassandra::cass_tuple_set_user_type;
pub use cassandra::cass_user_type_new_from_data_type;
pub use cassandra::cass_user_type_free;
pub use cassandra::cass_user_type_data_type;
pub use cassandra::cass_user_type_set_null;
pub use cassandra::cass_user_type_set_null_by_name;
pub use cassandra::cass_user_type_set_null_by_name_n;
pub use cassandra::cass_user_type_set_int8;
pub use cassandra::cass_user_type_set_int8_by_name;
pub use cassandra::cass_user_type_set_int8_by_name_n;
pub use cassandra::cass_user_type_set_int16;
pub use cassandra::cass_user_type_set_int16_by_name;
pub use cassandra::cass_user_type_set_int16_by_name_n;
pub use cassandra::cass_user_type_set_int32;
pub use cassandra::cass_user_type_set_int32_by_name;
pub use cassandra::cass_user_type_set_int32_by_name_n;
pub use cassandra::cass_user_type_set_uint32;
pub use cassandra::cass_user_type_set_uint32_by_name;
pub use cassandra::cass_user_type_set_uint32_by_name_n;
pub use cassandra::cass_user_type_set_int64;
pub use cassandra::cass_user_type_set_int64_by_name;
pub use cassandra::cass_user_type_set_int64_by_name_n;
pub use cassandra::cass_user_type_set_float;
pub use cassandra::cass_user_type_set_float_by_name;
pub use cassandra::cass_user_type_set_float_by_name_n;
pub use cassandra::cass_user_type_set_double;
pub use cassandra::cass_user_type_set_double_by_name;
pub use cassandra::cass_user_type_set_double_by_name_n;
pub use cassandra::cass_user_type_set_bool;
pub use cassandra::cass_user_type_set_bool_by_name;
pub use cassandra::cass_user_type_set_bool_by_name_n;
pub use cassandra::cass_user_type_set_string;
pub use cassandra::cass_user_type_set_string_n;
pub use cassandra::cass_user_type_set_string_by_name;
pub use cassandra::cass_user_type_set_string_by_name_n;
pub use cassandra::cass_user_type_set_bytes;
pub use cassandra::cass_user_type_set_bytes_by_name;
pub use cassandra::cass_user_type_set_bytes_by_name_n;
pub use cassandra::cass_user_type_set_uuid;
pub use cassandra::cass_user_type_set_uuid_by_name;
pub use cassandra::cass_user_type_set_uuid_by_name_n;
pub use cassandra::cass_user_type_set_inet;
pub use cassandra::cass_user_type_set_inet_by_name;
pub use cassandra::cass_user_type_set_inet_by_name_n;
pub use cassandra::cass_user_type_set_decimal;
pub use cassandra::cass_user_type_set_decimal_by_name;
pub use cassandra::cass_user_type_set_decimal_by_name_n;
pub use cassandra::cass_user_type_set_collection;
pub use cassandra::cass_user_type_set_collection_by_name;
pub use cassandra::cass_user_type_set_collection_by_name_n;
pub use cassandra::cass_user_type_set_tuple;
pub use cassandra::cass_user_type_set_tuple_by_name;
pub use cassandra::cass_user_type_set_tuple_by_name_n;
pub use cassandra::cass_user_type_set_user_type;
pub use cassandra::cass_user_type_set_user_type_by_name;
pub use cassandra::cass_user_type_set_user_type_by_name_n;
pub use cassandra::cass_result_free;
pub use cassandra::cass_result_row_count;
pub use cassandra::cass_result_column_count;
pub use cassandra::cass_result_column_name;
pub use cassandra::cass_result_column_type;
pub use cassandra::cass_result_column_data_type;
pub use cassandra::cass_result_first_row;
pub use cassandra::cass_result_has_more_pages;
pub use cassandra::cass_result_paging_state_token;
pub use cassandra::cass_error_result_free;
pub use cassandra::cass_error_result_code;
pub use cassandra::cass_error_result_consistency;
pub use cassandra::cass_error_result_responses_received;
pub use cassandra::cass_error_result_responses_required;
pub use cassandra::cass_error_result_num_failures;
pub use cassandra::cass_error_result_data_present;
pub use cassandra::cass_error_result_write_type;
pub use cassandra::cass_error_result_keyspace;
pub use cassandra::cass_error_result_table;
pub use cassandra::cass_error_result_function;
pub use cassandra::cass_error_num_arg_types;
pub use cassandra::cass_error_result_arg_type;
pub use cassandra::cass_iterator_free;
pub use cassandra::cass_iterator_type;
pub use cassandra::cass_iterator_from_result;
pub use cassandra::cass_iterator_from_row;
pub use cassandra::cass_iterator_from_collection;
pub use cassandra::cass_iterator_from_map;
pub use cassandra::cass_iterator_from_tuple;
pub use cassandra::cass_iterator_fields_from_user_type;
pub use cassandra::cass_iterator_keyspaces_from_schema_meta;
pub use cassandra::cass_iterator_tables_from_keyspace_meta;
pub use cassandra::cass_iterator_user_types_from_keyspace_meta;
pub use cassandra::cass_iterator_functions_from_keyspace_meta;
pub use cassandra::cass_iterator_aggregates_from_keyspace_meta;
pub use cassandra::cass_iterator_fields_from_keyspace_meta;
pub use cassandra::cass_iterator_columns_from_table_meta;
pub use cassandra::cass_iterator_fields_from_table_meta;
pub use cassandra::cass_iterator_fields_from_column_meta;
pub use cassandra::cass_iterator_fields_from_function_meta;
pub use cassandra::cass_iterator_fields_from_aggregate_meta;
pub use cassandra::cass_iterator_next;
pub use cassandra::cass_iterator_get_row;
pub use cassandra::cass_iterator_get_column;
pub use cassandra::cass_iterator_get_value;
pub use cassandra::cass_iterator_get_map_key;
pub use cassandra::cass_iterator_get_map_value;
pub use cassandra::cass_iterator_get_user_type_field_name;
pub use cassandra::cass_iterator_get_user_type_field_value;
pub use cassandra::cass_iterator_get_keyspace_meta;
pub use cassandra::cass_iterator_get_table_meta;
pub use cassandra::cass_iterator_get_user_type;
pub use cassandra::cass_iterator_get_function_meta;
pub use cassandra::cass_iterator_get_aggregate_meta;
pub use cassandra::cass_iterator_get_column_meta;
pub use cassandra::cass_iterator_get_meta_field_name;
pub use cassandra::cass_iterator_get_meta_field_value;
pub use cassandra::cass_row_get_column;
pub use cassandra::cass_row_get_column_by_name;
pub use cassandra::cass_row_get_column_by_name_n;
pub use cassandra::cass_value_data_type;
pub use cassandra::cass_value_get_int8;
pub use cassandra::cass_value_get_int16;
pub use cassandra::cass_value_get_int32;
pub use cassandra::cass_value_get_uint32;
pub use cassandra::cass_value_get_int64;
pub use cassandra::cass_value_get_float;
pub use cassandra::cass_value_get_double;
pub use cassandra::cass_value_get_bool;
pub use cassandra::cass_value_get_uuid;
pub use cassandra::cass_value_get_inet;
pub use cassandra::cass_value_get_string;
pub use cassandra::cass_value_get_bytes;
pub use cassandra::cass_value_get_decimal;
pub use cassandra::cass_value_type;
pub use cassandra::cass_value_is_null;
pub use cassandra::cass_value_is_collection;
pub use cassandra::cass_value_item_count;
pub use cassandra::cass_value_primary_sub_type;
pub use cassandra::cass_value_secondary_sub_type;
pub use cassandra::cass_uuid_gen_new;
pub use cassandra::cass_uuid_gen_new_with_node;
pub use cassandra::cass_uuid_gen_free;
pub use cassandra::cass_uuid_gen_time;
pub use cassandra::cass_uuid_gen_random;
pub use cassandra::cass_uuid_gen_from_time;
pub use cassandra::cass_uuid_min_from_time;
pub use cassandra::cass_uuid_max_from_time;
pub use cassandra::cass_uuid_timestamp;
pub use cassandra::cass_uuid_version;
pub use cassandra::cass_uuid_string;
pub use cassandra::cass_uuid_from_string;
pub use cassandra::cass_uuid_from_string_n;
pub use cassandra::cass_timestamp_gen_server_side_new;
pub use cassandra::cass_timestamp_gen_monotonic_new;
pub use cassandra::cass_timestamp_gen_free;
pub use cassandra::cass_retry_policy_default_new;
pub use cassandra::cass_retry_policy_downgrading_consistency_new;
pub use cassandra::cass_retry_policy_fallthrough_new;
pub use cassandra::cass_retry_policy_logging_new;
pub use cassandra::cass_retry_policy_free;
pub use cassandra::cass_custom_payload_new;
pub use cassandra::cass_custom_payload_free;
pub use cassandra::cass_custom_payload_set;
pub use cassandra::cass_custom_payload_set_n;
pub use cassandra::cass_consistency_string;
pub use cassandra::cass_write_type_string;
pub use cassandra::cass_error_desc;
pub use cassandra::cass_log_cleanup;
pub use cassandra::cass_log_set_level;
pub use cassandra::cass_log_set_callback;
pub use cassandra::cass_log_set_queue_size;
pub use cassandra::cass_log_level_string;
pub use cassandra::cass_inet_init_v4;
pub use cassandra::cass_inet_init_v6;
pub use cassandra::cass_inet_string;
pub use cassandra::cass_inet_from_string;
pub use cassandra::cass_inet_from_string_n;
pub use cassandra::cass_date_from_epoch;
pub use cassandra::cass_time_from_epoch;
pub use cassandra::cass_date_time_to_epoch;

pub use cassandra::CassVersion;
pub use cassandra::CassIndexMeta;
pub use cassandra::CassMaterializedViewMeta;
pub use cassandra::cass_cluster_set_blacklist_dc_filtering_n;
pub use cassandra::cass_cluster_set_blacklist_dc_filtering;
pub use cassandra::cass_cluster_set_blacklist_filtering_n;
pub use cassandra::cass_cluster_set_blacklist_filtering;
pub use cassandra::cass_cluster_set_whitelist_dc_filtering_n;
pub use cassandra::cass_cluster_set_whitelist_dc_filtering;
pub use cassandra::cass_data_type_is_frozen;
pub use cassandra::cass_index_meta_field_by_name_n;
pub use cassandra::cass_index_meta_field_by_name;
pub use cassandra::cass_index_meta_name;
pub use cassandra::cass_index_meta_type;
pub use cassandra::cass_index_meta_target;
pub use cassandra::cass_index_meta_options;
pub use cassandra::cass_iterator_indexes_from_table_meta;
pub use cassandra::cass_iterator_materialized_views_from_table_meta;
pub use cassandra::cass_iterator_columns_from_materialized_view_meta;
pub use cassandra::cass_iterator_fields_from_materialized_view_meta;
pub use cassandra::cass_iterator_fields_from_index_meta;
pub use cassandra::cass_iterator_get_index_meta;
pub use cassandra::CassClusteringOrder;
pub use cassandra::cass_iterator_get_materialized_view_meta;
pub use cassandra::cass_iterator_materialized_views_from_keyspace_meta;
pub use cassandra::cass_keyspace_meta_materialized_view_by_name;
pub use cassandra::cass_keyspace_meta_materialized_view_by_name_n;
pub use cassandra::cass_table_meta_index_by_name;
pub use cassandra::cass_table_meta_index_by_name_n;
pub use cassandra::cass_table_meta_index_count;
pub use cassandra::cass_table_meta_index;
pub use cassandra::cass_table_meta_materialized_view_by_name;
pub use cassandra::cass_table_meta_materialized_view_by_name_n;
pub use cassandra::cass_schema_meta_version;
pub use cassandra::cass_table_meta_materialized_view_count;
pub use cassandra::cass_table_meta_materialized_view;
pub use cassandra::cass_table_meta_clustering_key_order;
pub use cassandra::cass_materialized_view_meta_column_by_name;
pub use cassandra::cass_materialized_view_meta_column_by_name_n;
pub use cassandra::cass_materialized_view_meta_name;
pub use cassandra::cass_materialized_view_meta_base_table;
pub use cassandra::cass_materialized_view_meta_column_count;
pub use cassandra::cass_materialized_view_meta_column;
pub use cassandra::cass_materialized_view_meta_partition_key_count;
pub use cassandra::cass_materialized_view_meta_partition_key;
pub use cassandra::cass_materialized_view_meta_clustering_key_count;
pub use cassandra::cass_materialized_view_meta_clustering_key;
pub use cassandra::cass_materialized_view_meta_clustering_key_order;
pub use cassandra::cass_materialized_view_meta_field_by_name;
pub use cassandra::cass_materialized_view_meta_field_by_name_n;


mod cassandra;
mod ffi_util;
