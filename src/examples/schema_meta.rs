#![allow(non_snake_case)]
extern crate cql_bindgen;

use std::mem;
use std::ffi::CString;

use cql_bindgen::*;

const CASS_UUID_STRING_LENGTH:usize = 37;

unsafe fn print_keyspace(session:&mut CassSession, keyspace:&str) {
    let schema = cass_session_get_schema(session);
    let keyspace = CString::new(keyspace).unwrap();
    let keyspace_meta = cass_schema_get_keyspace(schema, keyspace.as_ptr());

    if !keyspace_meta.is_null() {
        print_schema_meta(&*keyspace_meta, 0);
    } else {
        println!("Unable to find {:?} keyspace in the schema metadata", keyspace);
    }

    let keyspace_meta = cass_schema_get_keyspace(schema, keyspace.as_ptr());

    if !keyspace_meta.is_null() {
        print_schema_meta(&*keyspace_meta, 0);
    } else {
        println!("Unable to find {:?} keyspace in the schema metadata", keyspace);
    }
    cass_schema_free(schema);
}

unsafe fn print_table(session:&mut CassSession, keyspace:&str, table:&str) {
    let schema = cass_session_get_schema(session);
    let keyspace = CString::new(keyspace).unwrap();
    let keyspace_meta = cass_schema_get_keyspace(schema, keyspace.as_ptr());
    let table = CString::new(table).unwrap();

    if !keyspace_meta.is_null() {
        let table_meta = cass_schema_meta_get_entry(keyspace_meta, table.as_ptr());
        if !table_meta.is_null() {
            print_schema_meta(&*table_meta, 0);
        } else {
            println!("Unable to find {:?} table in the schema metadata", table);
        }
    } else {
        println!("Unable to find {:?} keyspace in the schema metadata", keyspace);
    }
    cass_schema_free(schema);
}

//unsafe fn print_error(future:&mut CassFuture) {
//    let message = cass_future_error_message(future);
//    println!("Error: {:?}", raw2utf8(message.data,message.length));
//}

unsafe fn execute_query(session:&mut CassSession, query:&str) -> u32 {
    let query = CString::new(query).unwrap();
    let statement = cass_statement_new(query.as_ptr(), 0);

    let future = cass_session_execute(session, statement);
    cass_future_wait(future);

    let rc = cass_future_error_code(future);
    if rc != CASS_OK {
       // print_error(&mut *future);
    }

    cass_future_free(future);
    cass_statement_free(statement);

    return rc;
}

fn main() {unsafe{
    let cluster = cass_cluster_new();
    let session = cass_session_new();
    let contact_points = CString::new("127.0.0.1").unwrap();
    cass_cluster_set_contact_points(cluster, contact_points.as_ptr());

    let connect_future = cass_session_connect(session, cluster);

    if cass_future_error_code(connect_future) == CASS_OK {

        execute_query(&mut*session,
                  "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \
                  'class': 'SimpleStrategy', 'replication_factor': '3' };");

        print_keyspace(&mut*session, "examples");

        execute_query(&mut*session,
                  "CREATE TABLE IF NOT EXISTS examples.schema_meta (key text, \
                  value bigint, \
                  PRIMARY KEY (key));");

        print_table(&mut *session, "examples", "schema_meta");

        /* Close the session */
        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);
    } else {
        /* Handle error */
        let mut m:*const i8 = mem::zeroed();
        let mut l = mem::zeroed();
        cass_future_error_message(connect_future,&mut m, &mut l);

        println!("Unable to connect: {:?}", raw2utf8(m,l as u64));
    }

    cass_future_free(connect_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}

fn print_indent(indent:u32) {
    for _ in 0..indent {
        print!("\t");
    }
}

unsafe fn print_schema_value(value:&CassValue) {
    let mut i:cass_int32_t=mem::zeroed();
    let mut b:cass_bool_t=mem::zeroed();
    let mut d:cass_double_t=mem::zeroed();
    //let mut s:CassString=mem::zeroed();
    let mut u:CassUuid=mem::zeroed();
    let mut us:[i8; CASS_UUID_STRING_LENGTH] = mem::zeroed();

    let cass_type = cass_value_type(value);
    match cass_type {
        CASS_VALUE_TYPE_INT => {
            cass_value_get_int32(value, &mut i);
            print!("{}", i);
        }

        CASS_VALUE_TYPE_BOOLEAN => {
            cass_value_get_bool(value, &mut b);
            print!("{}", if b > 0 {"true"} else {"false"});
        }   
    
        CASS_VALUE_TYPE_DOUBLE => {
            cass_value_get_double(value, &mut d);
            print!("{}", d);
        }

//        CASS_VALUE_TYPE_TEXT|CASS_VALUE_TYPE_ASCII|CASS_VALUE_TYPE_VARCHAR => {
//            cass_value_get_string(value, &mut s);
//
//            print!("{:?}", raw2utf8(s.data,s.length));
//        }
    
        CASS_VALUE_TYPE_UUID => {
            cass_value_get_uuid(value, &mut u);
            cass_uuid_string(u, &mut*us.as_mut_ptr());
            print!("{:?}", "us - FIXME" /*us*/);
        }
    
        CASS_VALUE_TYPE_LIST => {
            print_schema_list(value);
        }

        CASS_VALUE_TYPE_MAP => {
            print_schema_map(value);
        }
        _ => {
            if cass_value_is_null(value) > 0 {
                print!("null");
            } else {
                print!("<unhandled type>");
            }
        }
    }
}

unsafe fn print_schema_list(value:&CassValue) {
    let iterator = cass_iterator_from_collection(value);
    let mut is_first = cass_true;

    print!("[ ");
    while cass_iterator_next(iterator) > 0 {
        if !is_first > 0 {print!(", ")};
        print_schema_value(&*cass_iterator_get_value(iterator));
        is_first = cass_false;
    }
    print!(" ]");
    cass_iterator_free(iterator);
}

unsafe fn print_schema_map(value:&CassValue) {
    let iterator = cass_iterator_from_map(value);
    let mut is_first = cass_true;

    print!("[[ ");
    while cass_iterator_next(iterator) > 0 {
        if !is_first > 0 {} print!(", ");
        print_schema_value(&*cass_iterator_get_map_key(iterator));
        print!(" : ");
        print_schema_value(&*cass_iterator_get_map_value(iterator));
        is_first = cass_false;
    }
    print!(" ]]");
    cass_iterator_free(iterator);
}

unsafe fn print_schema_meta_field(field:&Struct_CassSchemaMetaField_, indent:u32) {
    //let name = cass_schema_meta_field_name(field);
    let value = cass_schema_meta_field_value(field);

    print_indent(indent);
  
   // print!("{:?}", raw2utf8(name.data,name.length));
    print_schema_value(&*value);
    print!("\n");
}

unsafe fn print_schema_meta_fields(meta:&CassSchemaMeta, indent:u32) {
    let fields = cass_iterator_fields_from_schema_meta(meta);

    while cass_iterator_next(fields) > 0 {
        print_schema_meta_field(&*cass_iterator_get_schema_meta_field(fields), indent);
    }
    cass_iterator_free(fields);
}

unsafe fn print_schema_meta_entries(meta:&CassSchemaMeta, indent:u32) {
    let entries = cass_iterator_from_schema_meta(meta);

    while cass_iterator_next(entries) > 0 {
        print_schema_meta(&*cass_iterator_get_schema_meta(entries), indent);
    }
    cass_iterator_free(entries);
}

//pub fn cass_value_get_string(value: *const CassValue,
//                                 output: *mut *const ::libc::c_char,
//                                 output_size: *mut size_t) -> CassError;

unsafe fn print_schema_meta(meta:&CassSchemaMeta, indent:u32) {
    
    print_indent(indent);
   	let mut output = mem::zeroed();
   	let mut output_size = mem::zeroed();

    match cass_schema_meta_type(meta) {
        CASS_SCHEMA_META_TYPE_KEYSPACE => {
            let KS_NAME = CString::new("keyspace_name").unwrap();
            let field = cass_schema_meta_get_field(meta, KS_NAME.as_ptr());
            cass_value_get_string(cass_schema_meta_field_value(&*field), &mut output, &mut output_size);

            println!("Keyspace {:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            print_schema_meta_entries(meta, indent + 1);
        }

        CASS_SCHEMA_META_TYPE_TABLE => {
            let CF_NAME = CString::new("columnfamily_name").unwrap();
            let field = cass_schema_meta_get_field(meta, CF_NAME.as_ptr());
            cass_value_get_string(cass_schema_meta_field_value(field), &mut output, &mut output_size);
      
            println!("Table {:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            print_schema_meta_entries(meta, indent + 1);
        }
    
        CASS_SCHEMA_META_TYPE_COLUMN => {
            let COLUMN_NAME = CString::new("column_name").unwrap();
            let field = cass_schema_meta_get_field(meta, COLUMN_NAME.as_ptr());
            cass_value_get_string(cass_schema_meta_field_value(field), &mut output, &mut output_size);

            println!("Column {:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            println!("");
        }
    _ => {panic!("")}
    }
}
