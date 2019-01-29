use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;



mod filedb;



fn main() {
    // db open
    let db = filedb::DB::open("my-db-file-path").unwrap();

    // db write
    db.put(&[1; 1]);
    db.put(&[44; 4]);
    db.put(&[55; 5]);
    db.put(b"dddddddddd");

    // db read
    for record in db.iterator() {
        println!("record: {:?}", record);
    }
}

