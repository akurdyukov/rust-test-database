#[macro_use]
extern crate criterion;

use std::fs::File;
use std::io::{BufRead, BufReader};

use criterion::Criterion;

const DONOR_FILE: &str = "donor-file";
const ROCKSDB_FILE: &str = "rocksdb_storage";
const EVENT_STORE_FILE: &str = "event_store_file";


#[derive(Debug)]
struct DBRow {
    db: rocksdb::DB,
    counter: u64,
    test_rows: Option<Vec<Vec<u8>>>,
}

fn bench_rocksdb_write(c: &mut Criterion) {
    let mut db_row = DBRow {
        db: rocksdb::DB::open_default(ROCKSDB_FILE).unwrap(),
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
        db: rocksdb::DB::open_default(ROCKSDB_FILE).unwrap(),
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


fn bench_rocksdb_init(c: &mut Criterion) {
    let _lines_len = fill_rocksdb();
    c.bench_function("rocksdb/init", |b| b.iter(||
        rocksdb::DB::open_default(ROCKSDB_FILE).unwrap()
    ));
}


#[derive(Debug)]
struct EventStoreWriter {
    db: event_store::Writer,
    counter: u64,
    test_rows: Option<Vec<Vec<u8>>>,
}


fn bench_event_store_write(c: &mut Criterion) {
    let db = event_store::EventStore::open(EVENT_STORE_FILE).unwrap();
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
    let db = event_store::EventStore::open(EVENT_STORE_FILE).unwrap();
    let db_reader = db.iterator().unwrap();

    let mut event_reader = EventStoreReader {
        iterator: db_reader,
    };

    c.bench_function("event_store/read", move |b| b.iter(|| event_store_read(&mut event_reader)));
}


fn event_store_read(db_row: &mut EventStoreReader) {
    let _res = &db_row.iterator.next().unwrap();
}


fn bench_event_store_init(c: &mut Criterion) {
    c.bench_function("event_store/init", |b| b.iter(||
        event_store::EventStore::open(EVENT_STORE_FILE).unwrap().write_module()
    ));
}


criterion_group!(
    benches,

    bench_rocksdb_write,
    bench_rocksdb_read,
    bench_rocksdb_init,

    bench_event_store_write,
    bench_event_store_read,
    bench_event_store_init
    );
criterion_main!(benches);


fn fill_rocksdb() -> usize {
    let lines = donor_file_lines();
    let lines_len = lines.len();
    let mut db_row = DBRow {
        db: rocksdb::DB::open_default(ROCKSDB_FILE).unwrap(),
        counter: 0,
        test_rows: Some(lines),
    };

    for _i in 0..lines_len {
        rocksdb_write(&mut db_row);
    }

    lines_len
}


fn donor_file_lines() -> Vec<Vec<u8>> {
    let increase_donor_lines: u32 = 1_000_000;

    let donor_lines = {
        let file = File::open(DONOR_FILE).unwrap();
        let reader = BufReader::new(file);
        let donor_lines: Vec<Vec<u8>> = reader.lines()
            .map(move |x| x.unwrap().as_bytes().to_owned())
            .collect();
        donor_lines
    };

    let mut lines: Vec<Vec<u8>> = vec![];
    for _ in 0..increase_donor_lines {
        let mut cloned_lines = donor_lines.clone();
        lines.append(&mut cloned_lines);
    }

    lines
}
