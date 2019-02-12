use crate as event_store;

#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::*;

    const TEST_FILE_1: &str = "test_store_file_1";
    const TEST_FILE_2: &str = "test_store_file_2";
    const TEST_FILE_3: &str = "test_store_file_3";

    fn event_store_write(file_name: &str, payload: &[u8]) -> Result<(), std::io::Error> {
        let db = event_store::EventStore::open(file_name).unwrap();
        let mut db_writer = db.write_module().unwrap();
        db_writer.put(payload)
    }

    fn event_store_last_record(file_name: &str) -> Option<Vec<u8>> {
        let db = event_store::EventStore::open(file_name).unwrap();
        let mut iterator = db.iterator().unwrap();
        iterator.next()
    }


    #[test]
    fn test_event_store_write() {
        let write_result = tests::event_store_write(TEST_FILE_1, b"test record");
        fs::remove_file(TEST_FILE_1).unwrap();
        assert!(write_result.is_ok());
    }

    #[test]
    fn test_event_store_read() {
        tests::event_store_write(TEST_FILE_2, b"test record").unwrap();
        let last_record = tests::event_store_last_record(TEST_FILE_2).unwrap();
        fs::remove_file(TEST_FILE_2).unwrap();
        assert_eq!(b"test record".to_vec(), last_record);
    }

    #[test]
    fn test_event_store_empty() {
        let last_record = tests::event_store_last_record(TEST_FILE_3);
        fs::remove_file(TEST_FILE_3).unwrap();
        assert_eq!(None, last_record);
    }
}
