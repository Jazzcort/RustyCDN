use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::prelude::*;

const MAX_FREQUENCY: i32 = 500;
pub struct CacheSystem {
    // Use hashmap to store the cache content
    cache: HashMap<String, Vec<u8>>,
    // Maximum size of the cache
    max_size: u64,
    // Current size of the cache
    cur_size: u64,
    // Use hashmap to track the request frequency of each content
    frequency: HashMap<String, i32>,
}

impl CacheSystem {
    // This function is used to create a new cache system with given maximum size
    pub fn new(max_size: u64) -> Self {
        CacheSystem {
            cache: HashMap::new(),
            max_size,
            cur_size: 0,
            frequency: HashMap::new(),
        }
    }

    // This function is used to add content to the cache system.
    // If the cache system is full, if will delete the least frequent content in the cache.
    pub fn add(&mut self, path: &str, content: &str) {
        self.add_frequency(path);

        if self.is_full() {
            let least_path = self.least_frequent(path);
            dbg!(&least_path);
            if least_path != path {
                self.delete_cache(&least_path);
                if !self.is_full() {
                    self.add_cache(path, content);
                }
            }
        } else {
            self.add_cache(path, content);
        }
    }

    // This function is used to retrieve the cache content.
    // It will be call when we know the content is in the cache system.
    pub fn get(&mut self, path: &str) -> String {
        let bytes = self.cache.get(path).unwrap();
        // Unzip the content
        let mut decoder = GzDecoder::new(bytes.as_slice());
        let mut s = String::new();
        decoder.read_to_string(&mut s).unwrap();
        self.add_frequency(path);
        s
    }

    // This function is used to delete contents in the cache system.
    fn delete_cache(&mut self, path: &str) {
        let deleted = self.cache.remove(path).unwrap();
        // Reduce the content size from current size of the cache system
        self.cur_size -= deleted.len() as u64;
    }

    // This function is used to add contents to the cache system.
    fn add_cache(&mut self, path: &str, content: &str) {
        // Zip the content
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(content.as_bytes()).unwrap();
        let bytes = encoder.finish().unwrap();
        // Add the content size to the current size of the cache system
        self.cur_size += bytes.len() as u64;
        self.cache.insert(path.to_string(), bytes);
    }

    // This function is used to add frequency of the given content.
    fn add_frequency(&mut self, path: &str) {
        let handle = self.frequency.entry(path.to_string()).or_insert(0);
        *handle += 1;
        // When frequency of the content hit maximum frequency, reset the frequency map
        if *handle > MAX_FREQUENCY {
            self.reset_frequency();
        }
    }

    // This function is used to find the least frequent content in the cache.
    fn least_frequent(&self, other: &str) -> String {
        let mut candidates: Vec<(String, i32)> = self
            .cache
            .keys()
            .map(|x| (x.to_string(), *self.frequency.get(x).unwrap()))
            .collect();
        // Add the new content to the candidates list to compare
        candidates.push((
            other.to_owned(),
            self.frequency.get(other).unwrap().to_owned(),
        ));
        let (content, _) = candidates.par_drain(..).reduce(
            || (String::new(), i32::MAX),
            |acc, x| {
                if x.1 < acc.1 {
                    x
                } else {
                    acc
                }
            },
        );

        content
    }

    // This function is used to check if the storage of the cache system is full.
    fn is_full(&self) -> bool {
        dbg!(self.cur_size, self.max_size);
        if self.cur_size >= self.max_size {
            true
        } else {
            false
        }
    }

    // This function is used to reset the frequency map.
    fn reset_frequency(&mut self) {
        // Calculate the sum of the frequency
        let fre_sum = self
            .frequency
            .values()
            .cloned()
            .par_bridge()
            .reduce(|| 0, |acc, x| acc + x);
        let keys: Vec<String> = self.frequency.keys().cloned().collect();
        for path in keys.into_iter() {
            // Calculate the new frequency based on its portion to the frequency sum
            let tmp = self.frequency.get(&path).unwrap().to_owned() * 100 / fre_sum;
            // If the new frequency is 0, remove it from frequency map and cache storage (if it exists in cache)
            if tmp <= 0 {
                self.frequency.remove(&path);
                if self.cache.contains_key(&path) {
                    self.delete_cache(&path);
                }
            } else {
                // Update the frequency to the new value
                let handle = self.frequency.get_mut(&path).unwrap();
                *handle = tmp;
            }
        }
    }

    // This function is used to check if the content exists in the cahce system.
    pub fn contains_key(&self, path: &str) -> bool {
        if self.cache.contains_key(path) {
            true
        } else {
            false
        }
    }
}
