use cassandra::CassError_;

error_chain! {
errors {
    LIB_BAD_PARAMS(t:CassError_){}
    LIB_NO_STREAMS (t:CassError_){}
    LIB_UNABLE_TO_INIT(t:CassError_){}
    LIB_MESSAGE_ENCODE(t:CassError_){}
    LIB_HOST_RESOLUTION(t:CassError_){}
    LIB_UNEXPECTED_RESPONSE(t:CassError_){}
    LIB_REQUEST_QUEUE_FULL(t:CassError_){}
    LIB_NO_AVAILABLE_IO_THREAD(t:CassError_){}
    LIB_WRITE_ERROR(t:CassError_){}
    LIB_NO_HOSTS_AVAILABLE(t:CassError_){}
    LIB_INDEX_OUT_OF_BOUNDS(t:CassError_){}
    LIB_INVALID_ITEM_COUNT(t:CassError_){}
    LIB_INVALID_VALUE_TYPE(t:CassError_){}
    LIB_REQUEST_TIMED_OUT(t:CassError_){}
    LIB_UNABLE_TO_SET_KEYSPACE(t:CassError_){}
    LIB_CALLBACK_ALREADY_SET(t:CassError_){}
    LIB_INVALID_STATEMENT_TYPE(t:CassError_){}
    LIB_NAME_DOES_NOT_EXIST(t:CassError_){}
    LIB_UNABLE_TO_DETERMINE_PROTOCOL(t:CassError_){}
    LIB_NULL_VALUE(t:CassError_){}
    LIB_NOT_IMPLEMENTED(t:CassError_){}
    LIB_UNABLE_TO_CONNECT(t:CassError_){}
    LIB_UNABLE_TO_CLOSE(t:CassError_){}
    LIB_NO_PAGING_STATE(t:CassError_){}
    LIB_PARAMETER_UNSET(t:CassError_){}
    LIB_INVALID_ERROR_RESULT_TYPE(t:CassError_){}
    LIB_INVALID_FUTURE_TYPE(t:CassError_){}
    LIB_INTERNAL_ERROR(t:CassError_){}
    LIB_INVALID_CUSTOM_TYPE(t:CassError_){}
    LIB_INVALID_DATA(t:CassError_){}
    LIB_NOT_ENOUGH_DATA(t:CassError_){}
    LIB_INVALID_STATE(t:CassError_){}
    LIB_NO_CUSTOM_PAYLOAD(t:CassError_){}
    SERVER_SERVER_ERROR(t:CassError_){}
    SERVER_PROTOCOL_ERROR(t:CassError_){}
    SERVER_BAD_CREDENTIALS(t:CassError_){}
    SERVER_UNAVAILABLE(t:CassError_){}
    SERVER_OVERLOADED(t:CassError_){}
    SERVER_IS_BOOTSTRAPPING(t:CassError_){}
    SERVER_TRUNCATE_ERROR(t:CassError_){}
    SERVER_WRITE_TIMEOUT(t:CassError_){}
    SERVER_READ_TIMEOUT(t:CassError_){}
    SERVER_READ_FAILURE(t:CassError_){}
    SERVER_FUNCTION_FAILURE(t:CassError_){}
    SERVER_WRITE_FAILURE(t:CassError_){}
    SERVER_SYNTAX_ERROR(t:CassError_){}
    SERVER_UNAUTHORIZED(t:CassError_){}
    SERVER_INVALID_QUERY(t:CassError_){}
    SERVER_CONFIG_ERROR(t:CassError_){}
    SERVER_ALREADY_EXISTS(t:CassError_){}
    SERVER_UNPREPARED(t:CassError_){}
    SSL_INVALID_CERT(t:CassError_){}
    SSL_INVALID_PRIVATE_KEY(t:CassError_){}
    SSL_NO_PEER_CERT(t:CassError_){}
    SSL_INVALID_PEER_CERT(t:CassError_){}
    SSL_IDENTITY_MISMATCH(t:CassError_){}
    SSL_PROTOCOL_ERROR(t:CassError_){}
    LAST_ENTRY(t:CassError_){}
    }
}

impl CassError_ {
    pub fn to_result<T>(self,t:T) -> Result<T> {
        use self::CassError_::*;
        match self {
            CASS_OK => Ok(T),
            CASS_ERROR_LIB_BAD_PARAMS => Err(ErrorKind::LIB_BAD_PARAMS(CASS_ERROR_LIB_BAD_PARAMS).into()),
            CASS_ERROR_LIB_NO_STREAMS => Err(ErrorKind::LIB_NO_STREAMS(CASS_ERROR_LIB_NO_STREAMS).into()),
            CASS_ERROR_LIB_UNABLE_TO_INIT => Err(ErrorKind::LIB_UNABLE_TO_INIT(CASS_ERROR_LIB_UNABLE_TO_INIT).into()),
            CASS_ERROR_LIB_MESSAGE_ENCODE => Err(ErrorKind::LIB_MESSAGE_ENCODE(CASS_ERROR_LIB_MESSAGE_ENCODE).into()),
            CASS_ERROR_LIB_HOST_RESOLUTION => {
                Err(ErrorKind::LIB_HOST_RESOLUTION(CASS_ERROR_LIB_HOST_RESOLUTION).into())
            }
            CASS_ERROR_LIB_UNEXPECTED_RESPONSE => {
                Err(ErrorKind::LIB_UNEXPECTED_RESPONSE(CASS_ERROR_LIB_UNEXPECTED_RESPONSE).into())
            }
            CASS_ERROR_LIB_REQUEST_QUEUE_FULL => {
                Err(ErrorKind::LIB_REQUEST_QUEUE_FULL(CASS_ERROR_LIB_UNEXPECTED_RESPONSE).into())
            }
            CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD => {
                Err(ErrorKind::LIB_NO_AVAILABLE_IO_THREAD(CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD).into())
            }
            CASS_ERROR_LIB_WRITE_ERROR => Err(ErrorKind::LIB_WRITE_ERROR(CASS_ERROR_LIB_WRITE_ERROR).into()),
            CASS_ERROR_LIB_NO_HOSTS_AVAILABLE => {
                Err(ErrorKind::LIB_NO_HOSTS_AVAILABLE(CASS_ERROR_LIB_NO_HOSTS_AVAILABLE).into())
            }
            CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS => {
                Err(ErrorKind::LIB_INDEX_OUT_OF_BOUNDS(CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS).into())
            }
            CASS_ERROR_LIB_INVALID_ITEM_COUNT => {
                Err(ErrorKind::LIB_INVALID_ITEM_COUNT(CASS_ERROR_LIB_INVALID_ITEM_COUNT).into())
            }
            CASS_ERROR_LIB_INVALID_VALUE_TYPE => {
                Err(ErrorKind::LIB_INVALID_VALUE_TYPE(CASS_ERROR_LIB_INVALID_VALUE_TYPE).into())
            }
            CASS_ERROR_LIB_REQUEST_TIMED_OUT => {
                Err(ErrorKind::LIB_REQUEST_TIMED_OUT(CASS_ERROR_LIB_REQUEST_TIMED_OUT).into())
            }
            CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE => {
                Err(ErrorKind::LIB_UNABLE_TO_SET_KEYSPACE(CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE).into())
            }
            CASS_ERROR_LIB_CALLBACK_ALREADY_SET => {
                Err(ErrorKind::LIB_CALLBACK_ALREADY_SET(CASS_ERROR_LIB_CALLBACK_ALREADY_SET).into())
            }
            CASS_ERROR_LIB_INVALID_STATEMENT_TYPE => {
                Err(ErrorKind::LIB_INVALID_STATEMENT_TYPE(CASS_ERROR_LIB_INVALID_STATEMENT_TYPE).into())
            }
            CASS_ERROR_LIB_NAME_DOES_NOT_EXIST => {
                Err(ErrorKind::LIB_NAME_DOES_NOT_EXIST(CASS_ERROR_LIB_NAME_DOES_NOT_EXIST).into())
            }
            CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL => {
                Err(ErrorKind::LIB_UNABLE_TO_DETERMINE_PROTOCOL(CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL).into())
            }
            CASS_ERROR_LIB_NULL_VALUE => Err(ErrorKind::LIB_NULL_VALUE(CASS_ERROR_LIB_NULL_VALUE).into()),
            CASS_ERROR_LIB_NOT_IMPLEMENTED => {
                Err(ErrorKind::LIB_NOT_IMPLEMENTED(CASS_ERROR_LIB_NOT_IMPLEMENTED).into())
            }
            CASS_ERROR_LIB_UNABLE_TO_CONNECT => {
                Err(ErrorKind::LIB_UNABLE_TO_CONNECT(CASS_ERROR_LIB_UNABLE_TO_CONNECT).into())
            }
            CASS_ERROR_LIB_UNABLE_TO_CLOSE => {
                Err(ErrorKind::LIB_UNABLE_TO_CLOSE(CASS_ERROR_LIB_UNABLE_TO_CLOSE).into())
            }
            CASS_ERROR_LIB_NO_PAGING_STATE => {
                Err(ErrorKind::LIB_NO_PAGING_STATE(CASS_ERROR_LIB_NO_PAGING_STATE).into())
            }
            CASS_ERROR_LIB_PARAMETER_UNSET => {
                Err(ErrorKind::LIB_PARAMETER_UNSET(CASS_ERROR_LIB_PARAMETER_UNSET).into())
            }
            CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE => {
                Err(ErrorKind::LIB_INVALID_ERROR_RESULT_TYPE(CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE).into())
            }
            CASS_ERROR_LIB_INVALID_FUTURE_TYPE => {
                Err(ErrorKind::LIB_INVALID_FUTURE_TYPE(CASS_ERROR_LIB_INVALID_FUTURE_TYPE).into())
            }
            CASS_ERROR_LIB_INTERNAL_ERROR => Err(ErrorKind::LIB_INTERNAL_ERROR(CASS_ERROR_LIB_INTERNAL_ERROR).into()),
            CASS_ERROR_LIB_INVALID_CUSTOM_TYPE => {
                Err(ErrorKind::LIB_INVALID_CUSTOM_TYPE(CASS_ERROR_LIB_INVALID_CUSTOM_TYPE).into())
            }
            CASS_ERROR_LIB_INVALID_DATA => Err(ErrorKind::LIB_INVALID_DATA(CASS_ERROR_LIB_INVALID_DATA).into()),
            CASS_ERROR_LIB_NOT_ENOUGH_DATA => {
                Err(ErrorKind::LIB_NOT_ENOUGH_DATA(CASS_ERROR_LIB_NOT_ENOUGH_DATA).into())
            }
            CASS_ERROR_LIB_INVALID_STATE => Err(ErrorKind::LIB_INVALID_STATE(CASS_ERROR_LIB_INVALID_STATE).into()),
            CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD => {
                Err(ErrorKind::LIB_NO_CUSTOM_PAYLOAD(CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD).into())
            }
            CASS_ERROR_SERVER_SERVER_ERROR => {
                Err(ErrorKind::SERVER_SERVER_ERROR(CASS_ERROR_SERVER_SERVER_ERROR).into())
            }
            CASS_ERROR_SERVER_PROTOCOL_ERROR => {
                Err(ErrorKind::SERVER_PROTOCOL_ERROR(CASS_ERROR_SERVER_PROTOCOL_ERROR).into())
            }
            CASS_ERROR_SERVER_BAD_CREDENTIALS => {
                Err(ErrorKind::SERVER_BAD_CREDENTIALS(CASS_ERROR_SERVER_BAD_CREDENTIALS).into())
            }
            CASS_ERROR_SERVER_UNAVAILABLE => Err(ErrorKind::SERVER_UNAVAILABLE(CASS_ERROR_SERVER_UNAVAILABLE).into()),
            CASS_ERROR_SERVER_OVERLOADED => Err(ErrorKind::SERVER_OVERLOADED(CASS_ERROR_SERVER_OVERLOADED).into()),
            CASS_ERROR_SERVER_IS_BOOTSTRAPPING => {
                Err(ErrorKind::SERVER_IS_BOOTSTRAPPING(CASS_ERROR_SERVER_IS_BOOTSTRAPPING).into())
            }
            CASS_ERROR_SERVER_TRUNCATE_ERROR => {
                Err(ErrorKind::SERVER_TRUNCATE_ERROR(CASS_ERROR_SERVER_TRUNCATE_ERROR).into())
            }
            CASS_ERROR_SERVER_WRITE_TIMEOUT => {
                Err(ErrorKind::SERVER_WRITE_TIMEOUT(CASS_ERROR_SERVER_WRITE_TIMEOUT).into())
            }
            CASS_ERROR_SERVER_READ_TIMEOUT => {
                Err(ErrorKind::SERVER_READ_TIMEOUT(CASS_ERROR_SERVER_READ_TIMEOUT).into())
            }
            CASS_ERROR_SERVER_READ_FAILURE => {
                Err(ErrorKind::SERVER_READ_FAILURE(CASS_ERROR_SERVER_READ_FAILURE).into())
            }
            CASS_ERROR_SERVER_FUNCTION_FAILURE => {
                Err(ErrorKind::SERVER_FUNCTION_FAILURE(CASS_ERROR_SERVER_FUNCTION_FAILURE).into())
            }
            CASS_ERROR_SERVER_WRITE_FAILURE => {
                Err(ErrorKind::SERVER_WRITE_FAILURE(CASS_ERROR_SERVER_WRITE_FAILURE).into())
            }
            CASS_ERROR_SERVER_SYNTAX_ERROR => {
                Err(ErrorKind::SERVER_SYNTAX_ERROR(CASS_ERROR_SERVER_WRITE_FAILURE).into())
            }
            CASS_ERROR_SERVER_UNAUTHORIZED => {
                Err(ErrorKind::SERVER_UNAUTHORIZED(CASS_ERROR_SERVER_UNAUTHORIZED).into())
            }
            CASS_ERROR_SERVER_INVALID_QUERY => {
                Err(ErrorKind::SERVER_INVALID_QUERY(CASS_ERROR_SERVER_INVALID_QUERY).into())
            }
            CASS_ERROR_SERVER_CONFIG_ERROR => {
                Err(ErrorKind::SERVER_CONFIG_ERROR(CASS_ERROR_SERVER_CONFIG_ERROR).into())
            }
            CASS_ERROR_SERVER_ALREADY_EXISTS => {
                Err(ErrorKind::SERVER_ALREADY_EXISTS(CASS_ERROR_SERVER_ALREADY_EXISTS).into())
            }
            CASS_ERROR_SERVER_UNPREPARED => Err(ErrorKind::SERVER_UNPREPARED(CASS_ERROR_SERVER_UNPREPARED).into()),
            CASS_ERROR_SSL_INVALID_CERT => Err(ErrorKind::SSL_INVALID_CERT(CASS_ERROR_SSL_INVALID_CERT).into()),
            CASS_ERROR_SSL_INVALID_PRIVATE_KEY => {
                Err(ErrorKind::SSL_INVALID_PRIVATE_KEY(CASS_ERROR_SSL_INVALID_PRIVATE_KEY).into())
            }
            CASS_ERROR_SSL_NO_PEER_CERT => Err(ErrorKind::SSL_NO_PEER_CERT(CASS_ERROR_SSL_NO_PEER_CERT).into()),
            CASS_ERROR_SSL_INVALID_PEER_CERT => {
                Err(ErrorKind::SSL_INVALID_PEER_CERT(CASS_ERROR_SSL_INVALID_PEER_CERT).into())
            }
            CASS_ERROR_SSL_IDENTITY_MISMATCH => {
                Err(ErrorKind::SSL_IDENTITY_MISMATCH(CASS_ERROR_SSL_IDENTITY_MISMATCH).into())
            }
            CASS_ERROR_SSL_PROTOCOL_ERROR => Err(ErrorKind::SSL_PROTOCOL_ERROR(CASS_ERROR_SSL_PROTOCOL_ERROR).into()),
            CASS_ERROR_LAST_ENTRY => Err(ErrorKind::LAST_ENTRY(CASS_ERROR_LAST_ENTRY).into()),
        }
    }
}
