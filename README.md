# lfu_rs

An LFU cache for Rust. This is an implementation of the algorithm described in [this paper](http://dhruvbird.com/lfu.pdf).

[![Build Status](https://travis-ci.org/mattusifer/lfu_rs.svg?branch=master)](https://travis-ci.org/mattusifer/lfu_rs)

### Usage

The `LFUCache` struct is similar to a typical [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

```rust
// Initialize cache with size 10
let mut cache: LFUCache<K, V> = LFUCache::new(10);

// Insertion
// Returns the old value if this key already existed
cache.insert(K, V) // -> Option<V>

// Retrieval
myValue = c.get(&K) // -> Option<&V>

// Removal
c.remove(&K) // Option<V>
```

### Running tests

```
$ cargo test --all
```
