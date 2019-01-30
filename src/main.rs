use std::fs::File;
use std::io::Read;

mod filedb;
mod bench;

fn main() {
    test_filedb();
    print!("\n");
    test_rocksdb();
}

fn test_filedb() {
    {
        let text = file_read_contents("donor-file");

        let _bench = bench::Bench::new("Test filedb open + write");
        // db open
        let db = filedb::DB::open("my-db-file-path").unwrap();
        let mut db_writer = db.write_module().unwrap();

        let _bench = bench::Bench::new("Test filedb        write");

        for line in text.lines() {
            let bytes = line.as_bytes();
            db_writer.put(bytes).unwrap();
        }
    }

    // db read
    let _bench = bench::Bench::new("Test filedb  open + read");
    // db open
    let db = filedb::DB::open("my-db-file-path").unwrap();
    let _bench = bench::Bench::new("Test filedb         read");

    for _record in db.iterator().unwrap() {
        // print!(".");
    }
}


fn test_rocksdb() {
    let mut i: u32 = 0;

    {
        let text = file_read_contents("donor-file");

        let _bench = bench::Bench::new("Test rocksdb open + write");

        // db open
        let db = rocksdb::DB::open_default("rocksdb_storage").unwrap();

        let _bench = bench::Bench::new("Test rocksdb        write");

        for line in text.lines() {
            let bytes = line.as_bytes();
            db.put(&i.to_be_bytes(), bytes).unwrap();
            i = i + 1;
        }
    }

    let _bench = bench::Bench::new("Test rocksdb  open + read");

    // db open
    let db = rocksdb::DB::open_default("rocksdb_storage").unwrap();

    let _bench = bench::Bench::new("Test rocksdb         read");

    for x in (0..i).rev() {
        let _res = db.get(&x.to_be_bytes()).unwrap().unwrap();
        //let res2 = res.to_utf8().unwrap();
        //println!("{}", res2);
    }
}


fn file_read_contents(file_path: &str) -> String {
    let mut f = File::open(file_path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}
