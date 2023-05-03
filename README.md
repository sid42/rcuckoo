# Cuckoo Filter implementation in Rust
As shown in https://www.cs.cmu.edu/~dga/papers/cuckoo-conext2014.pdf

A Cuckoo filter is a type of probabilistic data structure designed to efficiently query set membership. Instead of providing a definite yes or no response when asked if an element exists in a set, a Cuckoo filter returns either a definite no (meaning the element definitely does not exist in the set) or a ¯\\(ツ)/¯ (meaning the element might exist in the set). This allows for optimized space usage, as the number of bits needed to represent set membership for an element can be reduced from 128 bits (as in the case of UUIDs) to just 3-7 bits (8 bits in this implementation).

Cuckoo filters are especially useful in practical settings where large sets of elements need to be stored in application memory and queried for membership quickly, without making frequent network calls to a database or cache. For example, NoSQL databases often use Cuckoo filters to check if a requested key exists in the underlying data structures or disk. By forfeiting a definite yes or no response, Cuckoo and Bloom filters optimise for space efficiency.

## Usage
As seen in `tests/tests.rs`
```rust
use rcuckoo::Cuckoo;

#[test]
fn simple() {
    let mut c = Cuckoo::new(2_i32.pow(10), 4, 500);

    // insert hi
    assert_eq!(c.insert("hi".as_bytes()), Ok(()));
    assert_eq!(c.exists("hi".as_bytes()), true);

    // check if hello exists
    assert_eq!(c.exists("hello".as_bytes()), false);

    // check deletion
    assert_eq!(c.delete("hi".as_bytes()), Ok(()));
    assert_eq!(c.exists("hi".as_bytes()), false);

    // delete item that does not exist
    assert!(c.delete("hola".as_bytes()).is_err());
}
```

Cuckoo hashing: http://www.itu.dk/people/pagh/papers/cuckoo-undergrad.pdf

Bloom filter: https://llimllib.github.io/bloomfilter-tutorial/