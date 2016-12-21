// #![feature(plugin)]
// #![plugin(clippy)]

extern crate cassandra_sys;

mod examples_util;
use examples_util::*;
use std::ffi::CString;
use std::mem;
use cassandra_sys::*;

fn insert_into_udt(session: &mut CassSession, schema_meta: &CassSchemaMeta, uuid_gen: &mut CassUuidGen)
                   -> Result<(), CassError> {
    unsafe {
        let mut id_str: [i8; CASS_UUID_STRING_LENGTH as usize] = [0; CASS_UUID_STRING_LENGTH as usize];

        let query = "INSERT INTO examples.udt (id, address) VALUES (?, ?)";

        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 2);
        let mut id = mem::zeroed();
        cass_uuid_gen_time(uuid_gen, &mut id);
        cass_uuid_string(id, id_str[..].as_mut_ptr());

        let keyspace_meta = cass_schema_meta_keyspace_by_name(schema_meta, CString::new("examples").unwrap().as_ptr());

        let udt_address = cass_keyspace_meta_user_type_by_name(keyspace_meta,
                                                               CString::new("address").unwrap().as_ptr());

        let udt_phone = cass_keyspace_meta_user_type_by_name(keyspace_meta,
                                                             CString::new("phone_numbers").unwrap().as_ptr());

        match (udt_address.is_null(), udt_phone.is_null()) {
            (_, true) => panic!("phone is null"),
            (true, _) => panic!("address is null"),
            (false, false) => {
                let address = cass_user_type_new_from_data_type(udt_address);
                let phone = cass_collection_new(CASS_COLLECTION_TYPE_SET, 2);

                for i in 0..2 {
                    let phone_numbers = cass_user_type_new_from_data_type(udt_phone);
                    cass_user_type_set_int32_by_name(phone_numbers,
                                                     CString::new("phone1").unwrap().as_ptr(),
                                                     i + 1);
                    cass_user_type_set_int32_by_name(phone_numbers,
                                                     CString::new("phone2").unwrap().as_ptr(),
                                                     i + 2);
                    cass_collection_append_user_type(phone, phone_numbers);
                    cass_user_type_free(phone_numbers);
                }

                cass_user_type_set_string_by_name(address,
                                                  CString::new("street").unwrap().as_ptr(),
                                                  id_str[..].as_mut_ptr());
                cass_user_type_set_string_by_name(address,
                                                  CString::new("city").unwrap().as_ptr(),
                                                  id_str[..].as_mut_ptr());
                cass_user_type_set_int32_by_name(address,
                                                 CString::new("zip").unwrap().as_ptr(),
                                                 id.time_and_version as i32);
                cass_user_type_set_collection_by_name(address, CString::new("phone").unwrap().as_ptr(), phone);

                cass_statement_bind_uuid(statement, 0, id);
                cass_statement_bind_user_type(statement, 1, address);

                let future = &mut *cass_session_execute(session, statement);
                cass_future_wait(future);

                match cass_future_error_code(future) {
                    CASS_OK => {}
                    _ => print_error(future),
                }

                cass_future_free(future);
                cass_user_type_free(address);
                cass_collection_free(phone);
            }
        }

        cass_statement_free(statement);

        Ok(())
    }
}

fn select_from_udt(session: &mut CassSession) -> Result<(), CassError> {
    unsafe {

        let query = "SELECT * FROM examples.udt";

        let statement = cass_statement_new(CString::new(query).unwrap().as_ptr(), 0);

        let future = &mut *cass_session_execute(session, statement);
        cass_future_wait(future);

        match cass_future_error_code(future) {
            CASS_OK => {
                let result = cass_future_get_result(future);
                let rows = cass_iterator_from_result(result);

                while cass_iterator_next(rows) == cass_true {
                    let mut id_str: [i8; CASS_UUID_STRING_LENGTH as usize] = [0; CASS_UUID_STRING_LENGTH as usize];
                    let row = cass_iterator_get_row(rows);
                    let id_value = cass_row_get_column_by_name(row, CString::new("id").unwrap().as_ptr());
                    let address_value = cass_row_get_column_by_name(row, CString::new("address").unwrap().as_ptr());
                    let fields = cass_iterator_fields_from_user_type(address_value);
                    let mut id = mem::zeroed();
                    cass_value_get_uuid(id_value, &mut id);
                    cass_uuid_string(id, id_str[..].as_mut_ptr());

                    println!("id {:?} ", id_str[..].as_mut_ptr());

                    while !fields.is_null() && cass_iterator_next(fields) == cass_true {
                        let mut field_name = mem::zeroed();
                        let mut field_name_length = mem::zeroed();
                        cass_iterator_get_user_type_field_name(fields, &mut field_name, &mut field_name_length);
                        let field_value = cass_iterator_get_user_type_field_value(fields);
                        println!("{:?} ", raw2utf8(field_name, field_name_length));

                        match cass_value_is_null(field_value) {
                            cass_false => {
                                match cass_value_type(field_value) {
                                    CASS_VALUE_TYPE_VARCHAR => {
                                        let mut text = mem::zeroed();
                                        let mut text_length = mem::zeroed();
                                        cass_value_get_string(field_value, &mut text, &mut text_length);
                                        println!("\"{:?}\" ", raw2utf8(text, text_length));
                                    }
                                    CASS_VALUE_TYPE_INT => {
                                        let mut i = mem::zeroed();
                                        cass_value_get_int32(field_value, &mut i);
                                        println!("{:?} ", i);
                                    }
                                    CASS_VALUE_TYPE_SET => {
                                        let phone_numbers = cass_iterator_from_collection(field_value);
                                        while cass_iterator_next(phone_numbers) == cass_true {
                                            let phone_value = cass_iterator_get_value(phone_numbers);
                                            let phone_fields = cass_iterator_fields_from_user_type(phone_value);
                                            assert!(cass_value_type(phone_value) == CASS_VALUE_TYPE_UDT);
                                            while cass_iterator_next(phone_fields) == cass_true {
                                                let phone_number_value =
                                                    cass_iterator_get_user_type_field_value(phone_fields);
                                                let mut i = mem::zeroed();
                                                cass_value_get_int32(phone_number_value, &mut i);
                                                println!("{:?} ", i);
                                            }
                                        }
                                    }
                                    _ => print!("<invalid> "),
                                }
                            }
                            cass_true => print!("<null> "),
                        }


                        println!("");
                    }

                    cass_result_free(result);
                    cass_iterator_free(rows);
                }

            }
            _ => print_error(future),
        }

        cass_future_free(future);
        cass_statement_free(statement);

        Ok(())
    }
}

fn main() {
    unsafe {
        let cluster = create_cluster();
        let session = &mut *cass_session_new();

        let uuid_gen = &mut *cass_uuid_gen_new();

        match connect_session(session, cluster) {
            Ok(_) => {}
            _ => {
                cass_cluster_free(cluster);
                cass_session_free(session);
                panic!();
            }
        }

        execute_query(session,
                      "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                       'replication_factor': '3' }")
            .unwrap();

        execute_query(session,
                      "CREATE TYPE IF NOT EXISTS examples.phone_numbers (phone1 int, phone2 int)")
            .unwrap();

        execute_query(session,
                      "CREATE TYPE IF NOT EXISTS examples.address (street text, city text, zip int, phone \
                       set<frozen<phone_numbers>>)")
            .unwrap();

        execute_query(session,
                      "CREATE TABLE IF NOT EXISTS examples.udt (id timeuuid, address frozen<address>, PRIMARY \
                       KEY(id))")
            .unwrap();

        let schema_meta = cass_session_get_schema_meta(session);

        insert_into_udt(session, &*schema_meta, uuid_gen).unwrap();
        select_from_udt(session).unwrap();

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

        cass_uuid_gen_free(uuid_gen);
    }
}
