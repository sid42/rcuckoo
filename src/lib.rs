pub mod bucket;

use std::hash::Hasher;
use bucket::Bucket;
use rand::Rng; 
use rustc_hash::FxHasher;

pub struct Cuckoo {
    buckets: Vec<Bucket>, // m buckets containing b entries
    num_buckets: i32, 
    max_kicks: i32,
}

impl Cuckoo {
    pub fn new(
        num_buckets: i32, 
        entries_per_bucket: i32,
        max_kicks: i32,
    ) -> Self {
        Self {
            num_buckets: num_buckets, 
            buckets: vec![Bucket::new(entries_per_bucket); num_buckets as usize],
            max_kicks: max_kicks,
        }
    }

    pub fn insert(&mut self, x: &[u8]) -> Result<(), &str> {
        let mut rng = rand::thread_rng();
        let f = hash(x) as u8; 
        let i1 = hash(x).rem_euclid(self.num_buckets);
        let i2 = (i1 ^ hash(&[f])).rem_euclid(self.num_buckets);

        if self.buckets[i1 as usize].insert(f) || self.buckets[i2 as usize].insert(f) {
            return Ok(());
        }

        let mut i = [i1, i2][rng.gen_range(0..2)];
        for _ in 0..self.max_kicks {
            let e = self.buckets[i as usize].swap(f);
            i = (i ^ hash(&[e])).rem_euclid(self.num_buckets);
            println!("i {} {}", i, i as usize); 
            if self.buckets[i as usize].insert(e) {
                return Ok(());
            }
        }

        return Err("Table full, need to rehash");
    }

    pub fn exists(&self, x: &[u8]) -> bool {
        let f = hash(x) as u8;
        let i1 = hash(x).rem_euclid(self.num_buckets);
        let i2 = (i1 ^ hash(&[f])).rem_euclid(self.num_buckets);

        return self.buckets[i1 as usize].lookup(f) || self.buckets[i2 as usize].lookup(f);
    }

    pub fn delete(&mut self, x: &[u8]) -> Result<(), &str> {
        let f = hash(x) as u8;
        let i1 = hash(x).rem_euclid(self.num_buckets);
        let i2 = (i1 ^ hash(&[f])).rem_euclid(self.num_buckets);
        
        if self.buckets[i1 as usize].lookup(f) {
            self.buckets[i1 as usize].delete(f);
            return Ok(());
        } else if self.buckets[i2 as usize].lookup(f) {
            self.buckets[i2 as usize].delete(f);
            return Ok(());
        }

        return Err("Deletion failed");
    }

}

fn hash(x: &[u8]) -> i32 { 
    let mut hasher = FxHasher::default();
    hasher.write(x);
    return hasher.finish() as i32;
}