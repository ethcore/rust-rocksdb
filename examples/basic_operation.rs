// Copyright 2014 Tyler Neely
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
extern crate parity_rocksdb as rocksdb;
use rocksdb::{DB, MergeOperands, Options, Writable};

fn main() {
    let path = "/tmp/rust-rocksdb";
    let db = DB::open_default(path).unwrap();
    assert!(db.put(b"my key", b"my value").is_ok());
    match db.get(b"my key") {
        Ok(Some(value)) => {
            match value.to_utf8() {
                Some(v) => println!("retrieved utf8 value: {}", v),
                None => println!("did not read valid utf-8 out of the db"),
            }
        }
        Ok(None) => panic!("value not present!"),
        Err(e) => println!("error retrieving value: {}", e),
    }

    assert!(db.delete(b"my key").is_ok());

    custom_merge();
}

fn concat_merge(_: &[u8],
                existing_val: Option<&[u8]>,
                operands: &mut MergeOperands)
                -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(operands.size_hint().0);
    match existing_val {
        Some(v) => {
            for e in v {
                result.push(*e)
            }
        }
        None => (),
    }
    for op in operands {
        for e in op {
            result.push(*e);
        }
    }
    result
}

fn custom_merge() {
    let path = "_rust_rocksdb_mergetest";
    let mut opts = Options::new();
    opts.create_if_missing(true);
    opts.add_merge_operator("test operator", concat_merge);
    {
        let db = DB::open(&opts, path).unwrap();
        db.put(b"k1", b"a").unwrap();
        db.merge(b"k1", b"b").unwrap();
        db.merge(b"k1", b"c").unwrap();
        db.merge(b"k1", b"d").unwrap();
        db.merge(b"k1", b"efg").unwrap();
        db.merge(b"k1", b"h").unwrap();
        match db.get(b"k1") {
            Ok(Some(value)) => {
                match value.to_utf8() {
                    Some(v) => println!("retrieved utf8 value: {}", v),
                    None => println!("did not read valid utf-8 out of the db"),
                }
            }
            Ok(None) => panic!("value not present!"),
            Err(e) => println!("error retrieving value: {}", e),
        }
    }
    let _ = DB::destroy(&opts, path).is_ok();
}


