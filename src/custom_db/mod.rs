use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Debug)]
pub enum Record {
    U32(u32),
    STRING(String),
}

impl From<u32> for Record {
    fn from(val: u32) -> Self {
        Record::U32(val)
    }
}

impl From<String> for Record {
    fn from(val: String) -> Self {
        Record::STRING(val)
    }
}

pub trait MakeRecord {
    fn new_record(self) -> Record;
}

impl MakeRecord for String {
    fn new_record(self) -> Record {
        Record::from(self)
    }
}

impl MakeRecord for u32 {
    fn new_record(self) -> Record {
        Record::from(self)
    }
}

impl Record {
    fn bytes_for_db(&self) -> Vec<u8> {
        let buf_val = self.val_to_bytes();
        let len = buf_val.len() as u64;
        let buf_len = len.to_be_bytes();

        let mut res_bytes = vec![];
        res_bytes.push(self.get_type_id());
        res_bytes.extend(&buf_len);
        res_bytes.extend(buf_val);
        res_bytes.extend(&buf_len);

        res_bytes
    }


    fn get_type_id(&self) -> u8 {
        match self {
            Record::U32(_) => 1,
            Record::STRING(_) => 2
        }
    }

    fn val_to_bytes(&self) -> Vec<u8> {
        match self {
            Record::U32(num) => num.to_be_bytes().to_vec(),
            Record::STRING(val) => val.clone().into_bytes()
        }
    }

    fn convert_from_bytes(type_id: u8, buf: Vec<u8>) -> Self {
        match type_id {
            2 => Record::from(String::from_utf8(buf).unwrap()),
            1 => {
                let mut array = [0; 4];
                let buf = &buf[..array.len()]; // panics if not enough data
                array.copy_from_slice(buf);

                let val = u32::from_be_bytes(array);
                Record::from(val)
            }
            _ => Record::from("not string".to_string()),
        }
    }
}


pub struct FileDB {
    file: File,
    offset: u64,
}


impl FileDB {
    pub fn new() -> Self {
        let file = File::open("db-file").unwrap();
        let file_len = file.metadata().unwrap().len();

        FileDB {
            file,
            offset: file_len,
        }
    }

    fn read_as_val_buf(&mut self, size: u64) -> Option<Vec<u8>> {
        // let mut buffer = Vec::with_capacity(size as usize);
        let mut buffer = vec![0; size as usize];
        (&self.file).read(&mut buffer).unwrap();

        Some(buffer)
    }


    fn read_as_val_size(&mut self) -> Option<u64> {
        let mut buffer = [0; 8];
        (&self.file).read(&mut buffer).unwrap();

        let len = u64::from_be_bytes(buffer);
        Some(len)
    }

    fn read_as_type_id(&mut self) -> Option<u8> {
        let mut buffer = [0];
        (&self.file).read(&mut buffer).unwrap();
        Some(buffer[0])
    }


    fn shift_offset_to_head(&mut self, step_bytes: u64) -> Option<()> {
        if self.offset >= step_bytes {
            self.offset -= step_bytes;
            (&self.file).seek(SeekFrom::Start(self.offset)).unwrap();
            Some(())
        } else {
            None
        }
    }


    pub fn add_record(make_rec: impl MakeRecord) {
        let val = make_rec.new_record();
        let rec_bytes = val.bytes_for_db();
        // println!("{:?}", &rec_bytes);
        FileDB::db_append_open().write(&rec_bytes).unwrap();
    }

    fn db_append_open() -> File {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open("db-file").unwrap();
        file
    }
}


impl Iterator for FileDB {
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        // val length
        self.shift_offset_to_head(8)?;
        let len = self.read_as_val_size().unwrap();
        // value
        self.shift_offset_to_head(len).unwrap();
        let val = self.read_as_val_buf(len).unwrap();
        // skip repeat of value length
        self.shift_offset_to_head(8).unwrap();
        // type
        self.shift_offset_to_head(1).unwrap();
        let type_id = self.read_as_type_id().unwrap();

        let record = Record::convert_from_bytes(type_id, val);
        Some(record)
    }
}
