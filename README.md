# Cuckoo Filter implementation in Rust
As shown in https://www.cs.cmu.edu/~dga/papers/cuckoo-conext2014.pdf

A Cuckoo filter is a probabilistic data structure which is used to query for set membership. When asked if an element exists in a set, a Cuckoo filter returns either a definite no (definitely does not exist in set) or a ¯\\_(ツ)_/¯ (element might exist in set). Cuckoo filters are optimised for space by forfeiting a definite yes / no response. In a practical setting, the number of bits to represent set membership for an element can be reduced from 128 bits (eg. UUID) to just 3-7 bits (8 bits in this implementation). A practical use of Cuckoo filters is in avoiding network calls to a DB/cache to check for set membership; large sets of elements can now be stored in application memory and membership can be queried much faster. (This technique is used in NoSQL databases to check if a requested key exists in the underlying data structures/disk)

## Usage
As seen in `tests/tests.rs`
```
use rcuckoo::Cuckoo;

#[test]
fn simple() {
    let mut c = Cuckoo::new(2_i32.pow(10), 4, 500);

    // we expect the insert to succeed, thus returning true
    assert_eq!(c.insert("hi".as_bytes()), true);
    assert_eq!(c.exists("hi".as_bytes()), true);
    assert_eq!(c.exists("hello".as_bytes()), false);
    // the delete operation should succeed
    assert_eq!(c.delete("hi".as_bytes()), true);
    assert_eq!(c.exists("hi".as_bytes()), false);
}
```

Cuckoo hashing: http://www.itu.dk/people/pagh/papers/cuckoo-undergrad.pdf

Bloom filter: https://llimllib.github.io/bloomfilter-tutorial/

## To-do
- Testing for false positives
