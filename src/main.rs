

mod custom_db;
use custom_db::FileDB;


fn main() {
    test_db_write();
    test_db_read();
}


fn test_db_write() {
    FileDB::add_record("test".to_string());
    FileDB::add_record("sdfsdf".to_string());
    FileDB::add_record(100);
}



fn test_db_read() {
    let db = FileDB::new();
    let iterator = db.into_iter();

    for record in iterator {
        println!("record: {:?}", record);
    }
}

