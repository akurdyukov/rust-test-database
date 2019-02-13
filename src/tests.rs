use std::fs;

use super::*;

const TEST_FILE_1: &str = "test_store_file_1";
const TEST_FILE_2: &str = "test_store_file_2";
const TEST_FILE_3: &str = "test_store_file_3";

#[test]
fn test_event_store_write() {
    // arrange
    let _ = fs::remove_file(TEST_FILE_1);
    let mut db = event_store::EventStore::open(TEST_FILE_1).unwrap();

    // act
    let result = db.put(b"test record");

    // assert
    assert!(result.is_ok());
    let _ = fs::remove_file(TEST_FILE_1);
}

#[test]
fn test_event_store_read() {
    // arrange
    let _ = fs::remove_file(TEST_FILE_2);
    let mut db = event_store::EventStore::open(TEST_FILE_2).unwrap();
    db.put(b"test record").ok();

    let mut iterator = db.iterator().unwrap();

    // act
    let item = iterator.next();

    // assert
    assert!(item.is_some());
    let value = item.unwrap();
    assert_eq!(b"test record".to_vec(), value);
    let _ = fs::remove_file(TEST_FILE_2);
}

#[test]
fn test_event_store_empty() {
    // arrange
    let _ = fs::remove_file(TEST_FILE_3);
    let db = event_store::EventStore::open(TEST_FILE_3).unwrap();
    let mut iterator = db.iterator().unwrap();

    // act
    let item = iterator.next();

    // assert
    assert!(item.is_none());
    let _ = fs::remove_file(TEST_FILE_3);
}