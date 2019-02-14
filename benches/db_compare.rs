use event_log_store::event_store;

pub trait DBWrapper<T> {
    fn new(db_path: &str) -> Self;
    fn fill_db(&mut self, val: &[u8], count: u64);
    fn put(&mut self, val: &[u8]);
    fn iterator(self) -> T;
}


#[derive(Debug)]
pub struct RocksDBWrapper {
    db: rocksdb::DB,
    pub counter: u64,
}


impl DBWrapper<RocksDBWrapper> for RocksDBWrapper {
    fn new(db_path: &str) -> Self {
        let db = rocksdb::DB::open_default(db_path).unwrap();
        RocksDBWrapper { db, counter: 0 }
    }

    fn fill_db(&mut self, val: &[u8], count: u64) {
        for _ in 0..count {
            self.put(val);
        }
    }

    fn put(&mut self, val: &[u8]) {
        let key = self.counter.to_be_bytes();
        self.db.put(&key, val).unwrap();
        self.counter = self.counter + 1;
    }

    fn iterator(self) -> Self {
        self
    }
}


impl Iterator for RocksDBWrapper {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut i = self.counter;

        if i == 0 {
            None
        } else {
            i = i - 1;
            self.counter = i;
            // TODO: maybe rewrite
            let _res_str = self.db.get(&i.to_be_bytes()).unwrap().unwrap().to_utf8().unwrap();
            Some(vec![])
        }
    }
}


#[derive(Debug)]
pub struct EventStoreWrapper {
    db: event_store::EventStore,
}

impl DBWrapper<event_store::StoreIterator> for EventStoreWrapper {
    fn new(db_path: &str) -> Self {
        let db = event_store::EventStore::open(db_path).unwrap();
        EventStoreWrapper { db }
    }
    fn fill_db(&mut self, val: &[u8], count: u64) {
        for _ in 0..count {
            self.put(val);
        }
    }

    fn put(&mut self, val: &[u8]) {
        self.db.put(val).unwrap()
    }

    fn iterator(self) -> event_store::StoreIterator {
        self.db.iterator().unwrap()
    }
}
