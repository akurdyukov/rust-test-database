#[macro_use]
extern crate criterion;

use std::fs::File;
use std::io::{BufRead, BufReader};


use criterion::Criterion;

const DONOR_FILE: &str = "donor-file";


#[derive(Debug)]
struct DBRow {
    db: rocksdb::DB,
    counter: u64,
    test_rows: Option<Vec<Vec<u8>>>,
}


fn bench_rocksdb_write(c: &mut Criterion) {
    let mut db_row = DBRow {
        db: rocksdb::DB::open_default("rocksdb_storage").unwrap(),
        counter: 0,
        test_rows: Some(donor_file_lines()),
    };

    c.bench_function("rocksdb/write", move |b| b.iter(|| rocksdb_write(&mut db_row)));
}

fn rocksdb_write(db_row: &mut DBRow) {
    let i = db_row.counter;
    db_row.counter = i + 1;

    let test_rows = db_row.test_rows.as_ref().unwrap();
    let val = test_rows[i as usize].as_ref();
    db_row.db.put(&i.to_be_bytes(), val).unwrap();
}


fn bench_rocksdb_read(c: &mut Criterion) {
    let lines_len = fill_rocksdb();

    let mut db_row = DBRow {
        db: rocksdb::DB::open_default("rocksdb_storage").unwrap(),
        counter: lines_len as u64,
        test_rows: None,
    };

    c.bench_function("rocksdb/read", move |b| b.iter(|| rocksdb_read(&mut db_row)));
}

fn rocksdb_read(db_row: &mut DBRow) {
    let i = db_row.counter;
    db_row.counter = i - 1;

    let _res = db_row.db.get(&i.to_be_bytes()).unwrap();
}


#[derive(Debug)]
struct EventStoreWriter {
    db: event_store::Writer,
    counter: u64,
    test_rows: Option<Vec<Vec<u8>>>,
}


fn bench_event_store_write(c: &mut Criterion) {
    let db = event_store::EventStore::open("my-db-file-path").unwrap();
    let db_writer = db.write_module().unwrap();

    let mut event_writer = EventStoreWriter {
        db: db_writer,
        counter: 0,
        test_rows: Some(donor_file_lines()),
    };

    c.bench_function("event_store/write", move |b| b.iter(|| event_store_write(&mut event_writer)));
}


fn event_store_write(db_row: &mut EventStoreWriter) {
    let i = db_row.counter;
    db_row.counter = i + 1;

    let test_rows = db_row.test_rows.as_ref().unwrap();
    let val = test_rows[i as usize].as_ref();

    db_row.db.put(val).unwrap();
}


struct EventStoreReader {
    iterator: event_store::StoreIterator,
}


fn bench_event_store_read(c: &mut Criterion) {
    // db open
    let db = event_store::EventStore::open("my-db-file-path").unwrap();
    let db_reader = db.iterator().unwrap();

    let mut event_reader = EventStoreReader {
        iterator: db_reader,
    };

    c.bench_function("event_store/read", move |b| b.iter(|| event_store_read(&mut event_reader)));
}


fn event_store_read(db_row: &mut EventStoreReader) {
    let _res = &db_row.iterator.next().unwrap();
}


criterion_group!(benches, bench_rocksdb_write, bench_rocksdb_read, bench_event_store_write, bench_event_store_read);
criterion_main!(benches);


fn fill_rocksdb() -> usize {
    let lines = donor_file_lines();
    let lines_len = lines.len();
    let mut db_row = DBRow {
        db: rocksdb::DB::open_default("rocksdb_storage").unwrap(),
        counter: 0,
        test_rows: Some(lines),
    };

    for _i in 0..lines_len {
        rocksdb_write(&mut db_row);
    }

    lines_len
}


fn donor_file_lines() -> Vec<Vec<u8>> {
    let file = File::open(DONOR_FILE).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<Vec<u8>> = reader.lines()
        .map(move |x| x.unwrap().as_bytes().to_owned())
        .collect();

    lines
}


#[test]
fn double_file_donor_content() {
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(DONOR_FILE).unwrap();
    let file_len = file.metadata().unwrap().len();

    let mut content: Vec<u8> = vec![0; file_len as usize];
    file.read_exact(&mut content).unwrap();
    file.write(&content).unwrap();
}
