#[derive(Clone)]
pub struct Bucket { 
    entries: Vec<u8>, // buckets are of size b
    b: i32, 
}

impl Bucket {
    pub fn new(bucket_size: i32) -> Self {
        Self {
            entries: vec![0; bucket_size as usize],
            b: bucket_size,
        }
    }

    pub fn insert(&mut self, f: u8) -> bool {
        if self.entries.len() >= self.b as usize {
            return false;
        } 

        self.entries.insert(0, f);
        return true
    }

    pub fn lookup(&self, f: u8) -> bool {
        for i in 0..self.entries.len() {
            if self.entries[i] == f {
                return true
            }
        }
        return false
    }

    pub fn delete(&mut self, f: u8) -> bool {
        for i in 0..self.entries.len() {
            if self.entries[i] == f {
                self.entries.swap_remove(i);
                return true;
            }
        }
        return false
    }

    pub fn get(self, f: u8) {

    }

    pub fn swap(&mut self, f: u8) -> u8 {
        let e = self.entries.swap_remove(0);
        self.entries.insert(0, f);
        return e
    }
}