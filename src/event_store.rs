use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

const DB_PAYLOAD_SIZE_LEN: u64 = 8;

#[derive(Debug)]
pub struct EventStore {
    path_buf: PathBuf,
    file: File,
}


impl EventStore {
    /// Open event store file for append
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path_buf = path.as_ref().to_owned();
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&path_buf)?;

        Ok(EventStore {
            path_buf,
            file,
        })
    }

    /// Write payload to storage
    pub fn put(&mut self, payload: &[u8]) -> Result<(), std::io::Error> {
        let len = payload.len();
        let buf_len = (len as u64).to_be_bytes();

        let mut record_bytes: Vec<u8> = Vec::with_capacity((DB_PAYLOAD_SIZE_LEN + DB_PAYLOAD_SIZE_LEN) as usize + len);
        record_bytes.extend_from_slice(&buf_len);
        record_bytes.extend_from_slice(payload);
        record_bytes.extend_from_slice(&buf_len);

        self.file.write(&record_bytes)?;

        Ok(())
    }

    /// Create read-only iterator of the storage.
    /// Iterates in reverse order starting from the end-of-file.
    pub fn iterator(&self) -> Result<StoreIterator, std::io::Error> {
        StoreIterator::create(self)
    }
}


pub struct StoreIterator {
    file: File,
    offset: u64,
}

impl StoreIterator {
    fn create(db: &EventStore) -> Result<Self, std::io::Error> {
        let file = File::open(&db.path_buf)?;
        let file_len = file.metadata()?.len();

        Ok(StoreIterator {
            file,
            offset: file_len,
        })
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