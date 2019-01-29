mod filedb;

fn main() {
    // db open
    let db = filedb::DB::open("my-db-file-path").unwrap();

    // db write
    db.put(&[1; 1]).unwrap();
    db.put(&[44; 4]).unwrap();
    db.put(&[55; 5]).unwrap();
    db.put(b"dddddddddd").unwrap();

    // db read
    for record in db.iterator() {
        println!("record: {:?}", record);
    }
}
