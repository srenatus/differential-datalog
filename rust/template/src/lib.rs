#![allow(
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    clippy::ptr_arg
)]

extern crate differential_dataflow;
extern crate fnv;
extern crate num_traits;
extern crate observe;
extern crate timely;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;
extern crate libc;
extern crate twox_hash;

#[macro_use]
extern crate differential_datalog;

#[macro_use]
extern crate abomonation;
extern crate ddlog_ovsdb_adapter;

#[cfg(feature = "flatbuf")]
extern crate flatbuffers;

use differential_dataflow::collection;
use timely::communication;
use timely::dataflow::scopes;
use timely::worker;

use abomonation::Abomonation;
use differential_datalog::arcval;
use differential_datalog::int::*;
use differential_datalog::program::*;
use differential_datalog::record;
use differential_datalog::record::{FromRecord, IntoRecord, Mutator};
use differential_datalog::uint::*;

use fnv::{FnvHashMap, FnvHashSet};
use libc::size_t;
use num_traits::identities::One;
use std::borrow;
use std::boxed;
use std::ffi;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;
use std::mem;
use std::ops::Deref;
use std::os::raw;
use std::os::unix;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::ptr;
use std::sync;

pub mod api;
pub mod ovsdb;
pub mod server;
pub mod update_handler;
pub mod valmap;

/* FlatBuffers bindings generated by `ddlog` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

/* FlatBuffers code generated by `flatc` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf_generated;

pub fn string_append_str(mut s1: String, s2: &str) -> String {
    s1.push_str(s2);
    s1
}

pub fn string_append(mut s1: String, s2: &String) -> String {
    s1.push_str(s2.as_str());
    s1
}

/*- !!!!!!!!!!!!!!!!!!!! -*/
// Don't edit this line
// Code below this point is needed to test-compile template
// code and is not part of the template.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Relations {
    X = 0,
}

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub enum Value {
    empty(),
    bool(bool),
}
unsafe_abomonate!(Value);

impl Default for Value {
    fn default() -> Value {
        Value::bool(false)
    }
}
impl fmt::Display for Value {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        panic!("Value::fmt not implemented")
    }
}

impl IntoRecord for Value {
    fn into_record(self) -> record::Record {
        panic!("Value::into_record not implemented")
    }
}

impl Mutator<Value> for record::Record {
    fn mutate(&self, _x: &mut Value) -> Result<(), String> {
        panic!("Value::mutate not implemented")
    }
}

pub fn relname2id(_rname: &str) -> Option<Relations> {
    panic!("relname2id not implemented")
}

pub fn output_relname_to_id(_rname: &str) -> Option<Relations> {
    panic!("output_relname_to_id not implemented")
}

pub fn input_relname_to_id(_rname: &str) -> Option<Relations> {
    panic!("input_relname_to_id not implemented")
}

pub fn record_update(_file: &mut fs::File, _upd: &record::UpdCmd) {
    panic!("record_update not implemented")
}

pub fn relid2rel(_rid: RelId) -> Option<Relations> {
    panic!("relid2rel not implemented")
}

pub fn relval_from_record(_rel: Relations, _rec: &record::Record) -> Result<Value, String> {
    panic!("relval_from_record not implemented")
}

pub fn relkey_from_record(_rel: Relations, _rec: &record::Record) -> Result<Value, String> {
    panic!("relkey_from_record not implemented")
}

pub fn relid2name(_rid: RelId) -> Option<&'static str> {
    panic!("relid2name not implemented")
}

pub fn prog(__update_cb: Box<dyn CBFn<Value>>) -> Program<Value> {
    panic!("prog not implemented")
}

lazy_static! {
    pub static ref RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}

lazy_static! {
    pub static ref INPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}

lazy_static! {
    pub static ref OUTPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}