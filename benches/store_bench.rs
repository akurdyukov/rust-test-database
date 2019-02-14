#[macro_use]
extern crate criterion;

use criterion::Criterion;

use event_log_store::event_store;

use crate::db_compare::DBWrapper;

pub mod db_compare;

const ROCKSDB_FILE: &str = "rocksdb_storage";
const EVENT_STORE_FILE: &str = "event_store_file";
const TEST_VALUE: &[u8] = &[0; 4096];
const FILL_DB_AMOUNT: u64 = 2_000_000;


fn bench_rocksdb_write(c: &mut Criterion) {
    // arrange
    let mut db_wrapper = db_compare::RocksDBWrapper::new(ROCKSDB_FILE);

    // act
    c.bench_function("rocksdb/write", move |b| b.iter(|| db_wrapper.put(TEST_VALUE)));
}


fn bench_rocksdb_read(c: &mut Criterion) {
    // arrange
    let mut db_wrapper = db_compare::RocksDBWrapper::new(ROCKSDB_FILE);
    db_wrapper.fill_db(TEST_VALUE, FILL_DB_AMOUNT);
    let mut rocks_iterator = db_wrapper.iterator();

    // act
    c.bench_function("rocksdb/read", move |b| b.iter(||
        match rocks_iterator.next() {
            Some(_) => (),
            None => panic!("rocksdb/read returned None"),
        }
    ));
}


fn bench_rocksdb_init(c: &mut Criterion) {
    c.bench_function("rocksdb/init", |b| b.iter(||
        rocksdb::DB::open_default(ROCKSDB_FILE).unwrap()
    ));
}


fn bench_event_store_write(c: &mut Criterion) {
    // arrange
    let mut db_wrapper = db_compare::EventStoreWrapper::new(EVENT_STORE_FILE);

    // act
    c.bench_function("event_store/write", move |b| b.iter(|| db_wrapper.put(TEST_VALUE)));
}


fn bench_event_store_read(c: &mut Criterion) {
    // arrange
    let mut db_wrapper = db_compare::EventStoreWrapper::new(EVENT_STORE_FILE);
    db_wrapper.fill_db(TEST_VALUE, FILL_DB_AMOUNT);
    let mut event_store_iterator = db_wrapper.iterator();

    // act
    c.bench_function("event_store/read", move |b| b.iter(||
        match event_store_iterator.next() {
            Some(_) => (),
            None => panic!("event_store/read returned None"),
        }
    ));
}


fn bench_event_store_init(c: &mut Criterion) {
    c.bench_function("event_store/init", |b| b.iter(||
        event_store::EventStore::open(EVENT_STORE_FILE).unwrap()
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
