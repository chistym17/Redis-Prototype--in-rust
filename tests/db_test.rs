use std::fs;
use redis_prototype::db::Database;
use redis_prototype::db::Value;


    #[test]
    fn test_insert_and_retrieve() {
        let mut db = Database::new();
        db.set("key1".to_string(), Value::String("value1".to_string()));
        let value = db.get("key1").unwrap();
        assert_eq!(value, &Value::String("value1".to_string()));
    }    

    #[test]
    fn test_save_and_load_snapshot() {
        let file_path = "test_db_snapshot.bin";
        let mut db = Database::new();  
        db.set("key2".to_string(),  Value::String("value2".to_string()));
        db.save_snaps(file_path).unwrap();
        let loaded_db = Database::load_snaps(file_path).unwrap();
        let loaded_value = loaded_db.get("key2").unwrap();
        assert_eq!(loaded_value, &Value::String("value2".to_string()));
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_nonexistent_snapshot() {
        let file_path = "nonexistent_snapshot.bin";
        let result = Database::load_snaps(file_path);
        assert!(result.is_err());
    }



