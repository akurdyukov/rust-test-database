mod filedb;

fn main() {
    // db open
    let db = filedb::DB::open("my-db-file-path").unwrap();

    let mut db_writer = db.write_module().unwrap();

    // db write
    db_writer.put(&[1; 1]).unwrap();
    db_writer.put(&[44; 4]).unwrap();
    db_writer.put(&[55; 5]).unwrap();
    db_writer.put(b"dddddddddd").unwrap();

    // db read
    for record in db.iterator().unwrap() {
        println!("record: {:?}", record);
    }
}
