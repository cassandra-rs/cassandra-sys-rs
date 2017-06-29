// #![feature(plugin)]
// #![plugin(clippy)]

#![allow(non_snake_case)]
extern crate cassandra_cpp_sys;

mod examples_util;
use examples_util::*;
use std::ffi::CString;
use std::mem;

use cassandra_cpp_sys::*;

const CASS_UUID_STRING_LENGTH: usize = 37;

fn print_indent(indent: u32) {
    for _ in 0..indent {
        print!("\t");
    }
}

unsafe fn print_schema_value(value: &CassValue) {
    let mut i: i32 = mem::zeroed();
    let mut b: cass_bool_t = mem::zeroed();
    let mut d: f64 = mem::zeroed();
    // let mut s:CassString=mem::zeroed();
    let mut u: CassUuid = mem::zeroed();
    let mut us: [i8; CASS_UUID_STRING_LENGTH] = mem::zeroed();

    match cass_value_type(value) {
        CASS_VALUE_TYPE_INT => {
            cass_value_get_int32(value, &mut i);
            println!("{}", i);
        }

        CASS_VALUE_TYPE_BOOLEAN => {
            cass_value_get_bool(value, &mut b);
            println!("{}",
                     match b {
                         cass_true => "true",
                         cass_false => "false",
                     });
        }

        CASS_VALUE_TYPE_DOUBLE => {
            cass_value_get_double(value, &mut d);
            println!("{}", d);
        }

        CASS_VALUE_TYPE_TEXT |
        CASS_VALUE_TYPE_ASCII |
        CASS_VALUE_TYPE_VARCHAR => {
            let mut s = mem::zeroed();
            let mut s_size = mem::zeroed();
            cass_value_get_string(value, &mut s, &mut s_size);
            println!("{}", raw2utf8(s, s_size).unwrap());
        }

        CASS_VALUE_TYPE_UUID => {
            cass_value_get_uuid(value, &mut u);
            cass_uuid_string(u, &mut *us.as_mut_ptr());
            println!("{}", "us - FIXME" /* us */);
        }

        CASS_VALUE_TYPE_LIST => {
            print_schema_list(value);
        }

        CASS_VALUE_TYPE_MAP => {
            print_schema_map(value);
        }
        _ => {
            match cass_value_is_null(value) {
                cass_true => println!("<unhandled type>: {:?}", cass_value_type(value)),
                cass_false => println!("null"),
            }
        }
    }
}

unsafe fn print_schema_list(value: &CassValue) {
    let iterator = cass_iterator_from_collection(value);
    let mut is_first = cass_true;

    print!("[ ");
    while cass_iterator_next(iterator) == cass_true {
        if is_first == cass_false {
            print!(", ")
        };
        print_schema_value(&*cass_iterator_get_value(iterator));
        is_first = cass_false;
    }
    print!(" ]");
    cass_iterator_free(iterator);
}

unsafe fn print_schema_map(value: &CassValue) {
    let iterator = cass_iterator_from_map(value);
    let mut is_first = cass_true;

    print!("[[ ");
    while cass_iterator_next(iterator) == cass_true {
        if is_first == cass_false {}
        print!(", ");
        print_schema_value(&*cass_iterator_get_map_key(iterator));
        print!(" : ");
        print_schema_value(&*cass_iterator_get_map_value(iterator));
        is_first = cass_false;
    }
    print!(" ]]");
    cass_iterator_free(iterator);
}

unsafe fn print_keyspace(session: &mut CassSession, keyspace: &str) {
    let schema_meta = cass_session_get_schema_meta(session);
    let keyspace_meta = cass_schema_meta_keyspace_by_name(schema_meta, CString::new(keyspace).unwrap().as_ptr());

    if !keyspace_meta.is_null() {
        print_keyspace_meta(&*keyspace_meta, 0);
    } else {
        println!("Unable to find {} keyspace in the schema metadata",
                 keyspace);
    }
    cass_schema_meta_free(schema_meta);
}

unsafe fn print_meta_fields(iterator: *mut CassIterator, indent: u32) {
    while cass_iterator_next(iterator) == cass_true {
        print_meta_field(iterator, indent);
    }
    cass_iterator_free(iterator);
}

unsafe fn print_meta_field(iterator: *const CassIterator, indent: u32) {
    let mut name = mem::zeroed();
    let mut name_length = mem::zeroed();
    cass_iterator_get_meta_field_name(iterator, &mut name, &mut name_length);
    let value = cass_iterator_get_meta_field_value(iterator);

    print_indent(indent);
    println!("{}: ", raw2utf8(name, name_length).unwrap());
    print_schema_value(&*value);
    println!("");
}


unsafe fn print_keyspace_meta(meta: *const CassKeyspaceMeta, indent: u32) {
    //  const char* name;
    //  size_t name_length;
    //  CassIterator* iterator;

    let mut name = mem::zeroed();
    let mut name_length = mem::zeroed();

    print_indent(indent);
    cass_keyspace_meta_name(meta, &mut name, &mut name_length);
    println!("Keyspace \"{}\":\n", raw2utf8(name, name_length).unwrap());

    print_meta_fields(cass_iterator_fields_from_keyspace_meta(meta), indent + 1);
    println!("");

    let iterator = cass_iterator_tables_from_keyspace_meta(meta);
    while cass_iterator_next(iterator) == cass_true {
        print_table_meta(cass_iterator_get_table_meta(iterator), indent + 1);
    }
    println!("");

    cass_iterator_free(iterator);
}


unsafe fn print_table_meta(meta: *const CassTableMeta, indent: u32) {
    let mut name = mem::zeroed();
    let mut name_length = mem::zeroed();


    print_indent(indent);
    cass_table_meta_name(meta, &mut name, &mut name_length);
    println!("Table \"{}\":", raw2utf8(name, name_length).unwrap());

    print_meta_fields(cass_iterator_fields_from_table_meta(meta), indent + 1);
    println!("");

    let iterator = cass_iterator_columns_from_table_meta(meta);
    while cass_iterator_next(iterator) == cass_true {
        print_column_meta(cass_iterator_get_column_meta(iterator), indent + 1);
    }
    println!("");

    cass_iterator_free(iterator);
}

unsafe fn print_column_meta(meta: *const CassColumnMeta, indent: u32) {

    let mut name = mem::zeroed();
    let mut name_length = mem::zeroed();

    print_indent(indent);
    cass_column_meta_name(meta, &mut name, &mut name_length);
    println!("Column \"{}\":", raw2utf8(name, name_length).unwrap());
    print_meta_fields(cass_iterator_fields_from_column_meta(meta), indent + 1);
    println!("");
}



unsafe fn print_table(session: &mut CassSession, keyspace: &str, table: &str) {
    let schema_meta = cass_session_get_schema_meta(session);
    let keyspace_meta = cass_schema_meta_keyspace_by_name(schema_meta, CString::new(keyspace).unwrap().as_ptr());

    if !keyspace_meta.is_null() {
        let table_meta = cass_keyspace_meta_table_by_name(keyspace_meta, CString::new(table).unwrap().as_ptr());
        if !table_meta.is_null() {
            print_table_meta(&*table_meta, 0);
        } else {
            println!("Unable to find {} table in the schemaname metadata", table);
        }
    } else {
        println!("Unable to find {} keyspace in the schema metadata",
                 keyspace);
    }
    cass_schema_meta_free(schema_meta);
}

pub fn main() {
    unsafe {
        let cluster = cass_cluster_new();
        let session = cass_session_new();
        cass_cluster_set_contact_points(cluster, CString::new("127.0.0.1").unwrap().as_ptr());

        let connect_future = cass_session_connect(session, cluster);

        match cass_future_error_code(connect_future) {
            CASS_OK => {

                execute_query(&mut *session,
                              "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': \
                               'SimpleStrategy', 'replication_factor': '3' };")
                    .unwrap();

                print_keyspace(&mut *session, "examples");

                execute_query(&mut *session,
                              "CREATE TABLE IF NOT EXISTS examples.schema_meta (key text, value bigint, PRIMARY KEY \
                               (key));")
                    .unwrap();

                print_table(&mut *session, "examples", "schema_meta");

                let close_future = cass_session_close(session);
                cass_future_wait(close_future);
                cass_future_free(close_future);
            }
            _ => {
                let mut m = mem::zeroed();
                let mut l = mem::zeroed();
                cass_future_error_message(connect_future, &mut m, &mut l);

                println!("Unable to connect: {}", raw2utf8(m, l).unwrap());
            }
        }
        cass_future_free(connect_future);
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
