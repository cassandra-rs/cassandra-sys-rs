// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

use std::mem;
use std::ffi::CString;

use cassandra_cpp_sys::*;

fn main() {
    unsafe {
        // Setup and connect to cluster
        let cluster = cass_cluster_new();
        let session = cass_session_new();

        // Add contact points
        cass_cluster_set_contact_points(cluster, CString::new("127.0.0.1").unwrap().as_ptr());

        // Provide the cluster object as configuration to connect the session
        let connect_future = cass_session_connect(session, cluster);

        let result = match cass_future_error_code(connect_future) {
            CASS_OK => {
                // Build statement and execute query
                let query = "SELECT keyspace_name FROM system_schema.keyspaces;";
                let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 0);

                let result_future = cass_session_execute(session, statement);

                match cass_future_error_code(result_future) {
                    CASS_OK => {
                        // Retrieve result set and iterate over the rows
                        let result = cass_future_get_result(result_future);
                        let rows = cass_iterator_from_result(result);

                        while cass_iterator_next(rows) == cass_true {
                            let row = cass_iterator_get_row(rows);
                            let value = cass_row_get_column_by_name(row,
                                                                    CString::new("keyspace_name").unwrap().as_ptr());

                            let mut keyspace_name = mem::zeroed();
                            let mut keyspace_name_length = mem::zeroed();
                            cass_value_get_string(value, &mut keyspace_name, &mut keyspace_name_length);
                            println!("keyspace_name: {:?}",
                                     raw2utf8(keyspace_name, keyspace_name_length).unwrap());
                        }

                        cass_result_free(result);
                        cass_iterator_free(rows);
                    }
                    rc => {
                        // Handle error
                        let mut message = mem::zeroed();
                        let mut message_length = mem::zeroed();
                        cass_future_error_message(result_future, &mut message, &mut message_length);
                        println!("Unable to run query: {:?}",
                                 raw2utf8(message, message_length));
                    }
                }

                cass_statement_free(statement);
                cass_future_free(result_future);

                // Close the session
                let close_future = cass_session_close(session);
                cass_future_wait(close_future);
                cass_future_free(close_future);
            }
            rc => {
                // Handle error
                let mut message = mem::zeroed();
                let mut message_length = mem::zeroed();
                cass_future_error_message(connect_future, &mut message, &mut message_length);
                println!("Unable to connect: {:?}", raw2utf8(message, message_length));
            }
        };



    };



}
