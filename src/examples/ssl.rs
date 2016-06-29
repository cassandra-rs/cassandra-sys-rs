// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_sys;

use std::mem;
use std::io::Result as IoResult;
use std::io::Read;
use std::fs::File;
use cassandra_sys::*;
use std::ffi::CString;

fn load_trusted_cert_file(file: &str, ssl: &mut CassSsl) -> IoResult<()> {
    unsafe {
        let mut file = try!(File::open(file));
        let cert_size = try!(file.metadata()).len() as usize;
        let mut cert: Vec<u8> = Vec::with_capacity(cert_size);
        let byte_len = try!(file.read_to_end(&mut cert));
        match byte_len == cert_size {
            true => {
                let rc = cass_ssl_add_trusted_cert_n(ssl, cert.as_ptr() as *const i8, cert_size);
                match rc {
                    CASS_OK => Ok(()),
                    rc => {
                        panic!("Error loading SSL certificate: {:?}", cass_error_desc(rc));
                    }
                }
            }
            false => {
                println!("Error loading SSL certificate. Not enough bytes read");
                Ok(())
            }
        }
    }
}

fn main() {
    unsafe {
        // Setup and connect to cluster
        let cluster = cass_cluster_new();
        let session = cass_session_new();
        let ssl = cass_ssl_new();

        cass_cluster_set_contact_points(cluster, CString::new("127.0.0.1").unwrap().as_ptr());

        // Only verify the certification and not the identity
        cass_ssl_set_verify_flags(ssl, CASS_SSL_VERIFY_PEER_CERT as i32);

        match load_trusted_cert_file("cert.pem", &mut *ssl) {
            Ok(_) => {}
            rc => {
                println!("Failed to load certificate disabling peer verification: {:?}",
                         rc);
                cass_ssl_set_verify_flags(ssl, CASS_SSL_VERIFY_NONE as i32);
            }
        }

        cass_cluster_set_ssl(cluster, ssl);

        let connect_future = cass_session_connect(session, cluster);

        match cass_future_error_code(connect_future) {
            CASS_OK => {

                // Build statement and execute query
                let query = "SELECT keyspace_name FROM system.schema_keyspaces;";
                let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 0);

                let result_future = cass_session_execute(session, statement);

                if cass_future_error_code(result_future) == CASS_OK {
                    // Retrieve result set and iterator over the rows
                    let result = cass_future_get_result(result_future);
                    let rows = cass_iterator_from_result(result);

                    while cass_iterator_next(rows) == cass_true {
                        let row = cass_iterator_get_row(rows);
                        let value = cass_row_get_column_by_name(row, CString::new("keyspace_name").unwrap().as_ptr());

                        let mut keyspace_name = mem::zeroed();
                        let mut keyspace_name_length = mem::zeroed();
                        cass_value_get_string(value, &mut keyspace_name, &mut keyspace_name_length);
                        println!("keyspace_name: {:?}",
                                 raw2utf8(keyspace_name, keyspace_name_length));
                    }

                    cass_result_free(result);
                    cass_iterator_free(rows);
                } else {
                    // Handle error
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    cass_future_error_message(result_future, &mut message, &mut message_length);
                    println!("Unable to run query: {:?}",
                             raw2utf8(message, message_length));
                }

                cass_statement_free(statement);
                cass_future_free(result_future);

                // Close the session
                let close_future = cass_session_close(session);
                cass_future_wait(close_future);
                cass_future_free(close_future);
            }
            _ => {
                // Handle error
                let mut message = mem::zeroed();
                let mut message_length = mem::zeroed();
                cass_future_error_message(connect_future, &mut message, &mut message_length);
                println!("Unable to connect: : {:?}",
                         raw2utf8(message, message_length));
            }
        }
        cass_future_free(connect_future);
        cass_cluster_free(cluster);
        cass_session_free(session);
        cass_ssl_free(ssl);
    }
}
