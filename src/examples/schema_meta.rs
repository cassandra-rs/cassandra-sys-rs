#![feature(plugin)]
#![plugin(clippy)]

#![allow(non_snake_case)]
extern crate cql_bindgen;

mod examples_util;
use examples_util::*;

use std::mem;

use cql_bindgen::*;

const CASS_UUID_STRING_LENGTH:usize = 37;

pub fn main() {
    unsafe {
        let cluster = cass_cluster_new();
        let session = cass_session_new();
        cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1"));

        let connect_future = cass_session_connect(session, cluster);

        if cass_future_error_code(connect_future) == CASS_OK {

            execute_query(&mut*session,
                          "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', \
                           'replication_factor': '3' };")
                .unwrap();

            print_keyspace(&mut*session, "examples");

            execute_query(&mut*session,
                          "CREATE TABLE IF NOT EXISTS examples.schema_meta (key text, value bigint, PRIMARY KEY \
                           (key));")
                .unwrap();

            print_table(&mut *session, "examples", "schema_meta");

            let close_future = cass_session_close(session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        } else {
        /* Handle error */
            let mut m = mem::zeroed();
            let mut l = mem::zeroed();
            cass_future_error_message(connect_future, &mut m, &mut l);

            println!("Unable to connect: {:?}", raw2utf8(m,l as u64));
        }

        cass_future_free(connect_future);
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}

fn print_indent(indent: u32) {
    for _ in 0..indent {
        print!("\t");
    }
}

unsafe fn print_schema_value(value: &CassValue) {
    let mut i: cass_int32_t = mem::zeroed();
    let mut b: cass_bool_t = mem::zeroed();
    let mut d: cass_double_t = mem::zeroed();
    //let mut s:CassString=mem::zeroed();
    let mut u: CassUuid = mem::zeroed();
    let mut us: [i8; CASS_UUID_STRING_LENGTH] = mem::zeroed();

    match cass_value_type(value) {
        CASS_VALUE_TYPE_INT => {
            cass_value_get_int32(value, &mut i);
            println!("{}", i);
        }

        CASS_VALUE_TYPE_BOOLEAN => {
            cass_value_get_bool(value, &mut b);
            println!("{}", if b > 0 {"true"} else {"false"});
        }

        CASS_VALUE_TYPE_DOUBLE => {
            cass_value_get_double(value, &mut d);
            println!("{}", d);
        }

        CASS_VALUE_TYPE_TEXT | CASS_VALUE_TYPE_ASCII | CASS_VALUE_TYPE_VARCHAR => {
            let mut s = mem::zeroed();
            let mut s_size = mem::zeroed();
            cass_value_get_string(value, &mut s, &mut s_size);
            println!("{:?}", raw2utf8(s,s_size));
        }

        CASS_VALUE_TYPE_UUID => {
            cass_value_get_uuid(value, &mut u);
            cass_uuid_string(u, &mut*us.as_mut_ptr());
            println!("{:?}", "us - FIXME" /*us*/);
        }

        CASS_VALUE_TYPE_LIST => {
            print_schema_list(value);
        }

        CASS_VALUE_TYPE_MAP => {
            print_schema_map(value);
        }
        _ => {
            match cass_value_is_null(value) {
                0 => println!("<unhandled type>: {:?}",cass_value_type(value)),
                _ => println!("null"),
            }
        }
    }
}

unsafe fn print_schema_list(value: &CassValue) {
    let iterator = cass_iterator_from_collection(value);
    let mut is_first = cass_true;

    print!("[ ");
    while cass_iterator_next(iterator) > 0 {
        if !is_first > 0 {
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
    while cass_iterator_next(iterator) > 0 {
        if !is_first > 0 {}
        print!(", ");
        print_schema_value(&*cass_iterator_get_map_key(iterator));
        print!(" : ");
        print_schema_value(&*cass_iterator_get_map_value(iterator));
        is_first = cass_false;
    }
    print!(" ]]");
    cass_iterator_free(iterator);
}

unsafe fn print_schema_meta_field(field: &Struct_CassSchemaMetaField_, indent: u32) {
    //let name = cass_schema_meta_field_name(field);
    let value = cass_schema_meta_field_value(field);

    print_indent(indent);

   // print!("{:?}", raw2utf8(name.data,name.length));
    print_schema_value(&*value);
    //print!("\n");
}

unsafe fn print_schema_meta_fields(meta: &CassSchemaMeta, indent: u32) {
    let fields = cass_iterator_fields_from_schema_meta(meta);

    while cass_iterator_next(fields) > 0 {
        print_schema_meta_field(&*cass_iterator_get_schema_meta_field(fields), indent);
    }
    cass_iterator_free(fields);
}

unsafe fn print_schema_meta_entries(meta: &CassSchemaMeta, indent: u32) {
    let entries = cass_iterator_from_schema_meta(meta);

    while cass_iterator_next(entries) > 0 {
        print_schema_meta(&*cass_iterator_get_schema_meta(entries), indent);
    }
    cass_iterator_free(entries);
}

unsafe fn print_schema_meta(meta: &CassSchemaMeta, indent: u32) {

    print_indent(indent);
    let mut output = mem::zeroed();
    let mut output_size = mem::zeroed();

    match cass_schema_meta_type(meta) {
        CASS_SCHEMA_META_TYPE_KEYSPACE => {
            let KS_NAME = "keyspace_name";
            let field = cass_schema_meta_get_field(meta, str2ref(KS_NAME));
            cass_value_get_string(cass_schema_meta_field_value(&*field), &mut output, &mut output_size);

            println!("Keyspace {:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            //println!("");
            print_schema_meta_entries(meta, indent + 1);
        }

        CASS_SCHEMA_META_TYPE_TABLE => {
            let CF_NAME = "columnfamily_name";
            let field = cass_schema_meta_get_field(meta, str2ref(CF_NAME));
            cass_value_get_string(cass_schema_meta_field_value(field), &mut output, &mut output_size);

            println!("Table {:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            //println!("");
            print_schema_meta_entries(meta, indent + 1);
        }

        CASS_SCHEMA_META_TYPE_COLUMN => {
            let COLUMN_NAME = "column_name";
            let field = cass_schema_meta_get_field(meta, str2ref(COLUMN_NAME));
            cass_value_get_string(cass_schema_meta_field_value(field), &mut output, &mut output_size);

            println!("{:?}", raw2utf8(output,output_size as u64));
            print_schema_meta_fields(meta, indent + 1);
            //println!("");
        }
        _ => {
            panic!("")
        }
    }
}

unsafe fn print_keyspace(session: &mut CassSession, keyspace: &str) {
    let schema = cass_session_get_schema(session);
    let keyspace_meta = cass_schema_get_keyspace(schema, str2ref(keyspace));

    if !keyspace_meta.is_null() {
        print_schema_meta(&*keyspace_meta, 0);
    } else {
        println!("Unable to find {:?} keyspace in the schema metadata", keyspace);
    }
    cass_schema_free(schema);
}

unsafe fn print_table(session: &mut CassSession, keyspace: &str, table: &str) {
    let schema = cass_session_get_schema(session);
    let keyspace_meta = cass_schema_get_keyspace(schema, str2ref(keyspace));

    if !keyspace_meta.is_null() {
        let table_meta = cass_schema_meta_get_entry(keyspace_meta, str2ref(table));
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
