extern crate lfu_rs;

use lfu_rs::LFUCache;

#[test]
fn insert_single_element() {
    let mut cache: LFUCache<String, String> = LFUCache::new(10);
    cache.insert("key1".to_string(), "val1".to_string());

    assert_eq!(cache.get(&"key1".to_string()), Some(&"val1".to_string()));
}

#[test]
fn insert_many_elements() {
    let size = 10;
    let mut cache: LFUCache<String, String> = LFUCache::new(10);

    for i in 0..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }

    for i in 0..size {
        assert_eq!(cache.get(&format!("key{}", i)), Some(&format!("val{}", i)))
    }
}

#[test]
fn structure_after_insertion() {
    let size = 10;
    let mut cache: LFUCache<String, String> = LFUCache::new(10);

    for i in 0..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }

    assert_eq!(
        cache.to_string(),
        "Count 1: val9 val8 val7 val6 val5 val4 val3 val2 val1 val0\n"
    );
}

#[test]
fn structure_after_retrieval() {
    let size = 10;
    let mut cache: LFUCache<String, String> = LFUCache::new(10);

    for i in 0..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }
    for i in 0..size {
        cache.get(&format!("key{}", i));
    }

    assert_eq!(
        cache.to_string(),
        "Count 2: val9 val8 val7 val6 val5 val4 val3 val2 val1 val0\n"
    );
}

#[test]
fn structure_after_removal() {
    let size = 10;
    let mut cache: LFUCache<String, String> = LFUCache::new(10);

    for i in 0..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }
    for i in 5..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }
    for i in 8..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }

    assert_eq!(cache.len(), 10);
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 2: val7 val6 val5",
        "Count 3: val9 val8"
    ));

    cache.remove(&"key6".to_string());
    assert_eq!(cache.len(), 9);
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 2: val7 val5",
        "Count 3: val9 val8"
    ));
    assert!(cache.get(&"key6".to_string()).is_none());

    cache.remove(&"key7".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 2: val5",
        "Count 3: val9 val8"
    ));

    cache.remove(&"key5".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 3: val9 val8"
    ));

    cache.remove(&"key4".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n",
        "Count 1: val3 val2 val1 val0",
        "Count 3: val9 val8"
    ));

    cache.insert("key4".to_string(), "val4".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 3: val9 val8"
    ));

    cache.remove(&"key8".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n{}\n",
        "Count 1: val4 val3 val2 val1 val0",
        "Count 3: val9"
    ));

    cache.remove(&"key9".to_string());
    assert_eq!(cache.to_string(), format!(
        "{}\n", "Count 1: val4 val3 val2 val1 val0"
    ));
}

#[test]
fn filling_in_gaps() {
    let size = 20;
    let mut cache: LFUCache<String, String> = LFUCache::new(size);

    for i in 0..size {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }

    cache.get(&"key5".to_string());
    cache.get(&"key5".to_string());

    for i in 5..size {
        cache.get(&format!("key{}", i));
    }

    assert_eq!(
        cache.to_string(),
        format!("{}{}{}",
                "Count 1: val4 val3 val2 val1 val0\n",
                "Count 2: val19 val18 val17 val16 val15 val14 val13 val12 val11 ",
                "val10 val9 val8 val7 val6\nCount 4: val5\n",
        )
    );

    cache.get(&"key6".to_string());

    assert_eq!(
        cache.to_string(),
        format!("{}{}{}",
                "Count 1: val4 val3 val2 val1 val0\n",
                "Count 2: val19 val18 val17 val16 val15 val14 val13 val12 val11 ",
                "val10 val9 val8 val7\nCount 3: val6\nCount 4: val5\n",
        )
    );

    cache.get(&"key6".to_string());

    assert_eq!(
        cache.to_string(),
        format!("{}{}{}",
                "Count 1: val4 val3 val2 val1 val0\n",
                "Count 2: val19 val18 val17 val16 val15 val14 val13 val12 val11 ",
                "val10 val9 val8 val7\nCount 4: val6 val5\n",
        )
    );
}

#[test]
fn overfill() {
    let size = 10;
    let mut cache: LFUCache<String, String> = LFUCache::new(size);

    for i in 0..size + 1 {
        cache.insert(format!("key{}", i), format!("val{}", i));
    }

    assert_eq!(
        cache.to_string(),
        format!("{}\n",
                "Count 1: val10 val8 val7 val6 val5 val4 val3 val2 val1 val0"
        )
    );

    cache.get(&"key10".to_string());
    cache.insert("key9".to_string(), "val9".to_string());

    assert_eq!(
        cache.to_string(),
        format!("{}\n{}\n",
                "Count 1: val9 val7 val6 val5 val4 val3 val2 val1 val0",
                "Count 2: val10"
        )
    );
}
