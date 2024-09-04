use redis_prototype::db::{Database, Value};
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

#[tokio::test]
async fn test_concurrency() {
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            tokio::time::sleep(Duration::from_millis(10)).await;

            {
                let mut db = db.lock().unwrap();
                db.set(key.clone(), Value::String(value.clone()));
            }

            tokio::time::sleep(Duration::from_millis(10)).await;

            {
                let db = db.lock().unwrap();
                assert_eq!(db.get(&key), Some(&Value::String(value)));
            }

            tokio::time::sleep(Duration::from_millis(10)).await;

            {
                let mut db = db.lock().unwrap();
                assert!(db.delete(&key));
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let db = db.lock().unwrap();
    assert!(db.is_empty());
}
