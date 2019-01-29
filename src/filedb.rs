use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

const DB_PAYLOAD_SIZE_LEN: u64 = 8;

pub struct DB {
    path_buf: PathBuf
}

impl DB {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path_buf = path.as_ref().to_owned();
        if !&path_buf.exists() {
            File::create(&path_buf)?;
        }
        Ok(DB { path_buf })
    }

    pub fn put(&self, payload: &[u8]) -> Result<(), std::io::Error> {
        let len = payload.len();
        let buf_len = (len as u64).to_be_bytes();

        let mut record_bytes: Vec<u8> = Vec::with_capacity((DB_PAYLOAD_SIZE_LEN + DB_PAYLOAD_SIZE_LEN) as usize + len);
        record_bytes.extend_from_slice(&buf_len);
        record_bytes.extend_from_slice(payload);
        record_bytes.extend_from_slice(&buf_len);

        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.path_buf)?;


        file.write(&record_bytes)?;

        Ok(())
    }

    pub fn iterator(&self) -> StoreIterator {
        StoreIterator::new(self)
    }
}


pub struct StoreIterator {
    file: File,
    offset: u64,
}

impl StoreIterator {
    fn new(db: &DB) -> Self {
        let file = File::open(&db.path_buf).unwrap();
        let file_len = file.metadata().unwrap().len();

        StoreIterator {
            file,
            offset: file_len,
        }
    }

    fn shift_offset_to_head(&mut self, shift: u64) -> () {
        if self.offset < shift {
            panic!("Database corrupted: offset is {}, require shift: {}", self.offset, shift)
        } else {
            self.offset -= shift;
        }
    }
}


impl Iterator for StoreIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        if self.offset == 0 {
            return None;
        }

        // payload length
        self.shift_offset_to_head(DB_PAYLOAD_SIZE_LEN);
        self.file.seek(SeekFrom::Start(self.offset)).unwrap();

        let mut buffer = [0; 8];
        self.file.read_exact(&mut buffer).unwrap();
        let len = u64::from_be_bytes(buffer);

        // payload
        self.shift_offset_to_head(len);
        self.file.seek(SeekFrom::Start(self.offset)).unwrap();
        let mut payload: Vec<u8> = vec![0; len as usize];
        self.file.read_exact(&mut payload).unwrap();

        // skip second payload length
        self.shift_offset_to_head(DB_PAYLOAD_SIZE_LEN);

        Some(payload)
    }
}
