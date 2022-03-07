// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_cpp_sys;

use cassandra_cpp_sys::*;
use std::env;
use std::ffi::CString;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <secure connect bundle zip> <username> <password>",
            args[0]
        );
        return;
    }

    let secure_connection_bundle = CString::new(args[1].as_str()).unwrap();
    let username = CString::new(args[2].as_str()).unwrap();
    let password = CString::new(args[3].as_str()).unwrap();

    unsafe {
        // create a cluster instance
        let cluster = cass_cluster_new();

        // set the secure connection bundle
        let result = cass_cluster_set_cloud_secure_connection_bundle(
            cluster,
            secure_connection_bundle.as_ptr(),
        );
        if result == CassError::CASS_OK {
            // set the credentials
            cass_cluster_set_credentials(cluster, username.as_ptr(), password.as_ptr());

            // create a database session
            let session = cass_session_new();

            // connect to the database
            let connect_future = cass_session_connect(session, cluster);
            let result = cass_future_error_code(connect_future);
            if result == CassError::CASS_OK {
                // create a statement
                let query = CString::new("SELECT release_version FROM system.local").unwrap();
                let statement = cass_statement_new(query.as_ptr(), 0);

                // execute statement
                let execute_future = cass_session_execute(session, statement);
                let result = cass_future_error_code(execute_future);
                if result == CassError::CASS_OK {
                    let dataset = cass_future_get_result(execute_future);
                    // get the first row
                    let row = cass_result_first_row(dataset);

                    // extract release_version column value
                    let name = CString::new("release_version").unwrap();
                    let value = cass_row_get_column_by_name(row, name.as_ptr());
                    let mut buffer = mem::zeroed();
                    let mut length = mem::zeroed();
                    cass_value_get_string(value, &mut buffer, &mut length);
                    println!("release_version: {}", raw2utf8(buffer, length).unwrap());

                    cass_result_free(dataset);
                } else {
                    eprintln!("Failed to execure statement: {}", result as u32);
                }

                cass_future_free(execute_future);

                cass_statement_free(statement);
            } else {
                eprintln!("Failed to connect to the database: {}", result as u32);
            }
            cass_future_free(connect_future);

            cass_session_free(session);
        } else {
            eprintln!("Failed to set secure connection bundle: {}", result as u32);
        }

        cass_cluster_free(cluster);
    }
}
