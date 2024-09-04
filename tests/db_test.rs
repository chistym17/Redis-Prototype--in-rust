use std::fs;
use redis_prototype::db::Database;
use redis_prototype::db::Value;


    #[test]
    fn test_insert_and_retrieve() {
        let mut db = Database::new();

        // Insert a value
        db.set("key1".to_string(), Value::String("value1".to_string()));
        // Retrieve and check the value
        let value = db.get("key1").unwrap();
        assert_eq!(value, &Value::String("value1".to_string()));
    }    

    #[test]
    fn test_save_and_load_snapshot() {
        let file_path = "test_db_snapshot.bin";
        let mut db = Database::new();
        
        // Insert a value
        db.set("key2".to_string(),  Value::String("value2".to_string()));

        // Save the snapshot
        db.save_snaps(file_path).unwrap();

        // Load the snapshot
        let loaded_db = Database::load_snaps(file_path).unwrap();

        // Verify the loaded data
        let loaded_value = loaded_db.get("key2").unwrap();
        assert_eq!(loaded_value, &Value::String("value2".to_string()));

        // Clean up the test file
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_nonexistent_snapshot() {
        let file_path = "nonexistent_snapshot.bin";

        // Attempt to load a non-existent snapshot
        let result = Database::load_snaps(file_path);

        // Verify that an error is returned
        assert!(result.is_err());
    }



