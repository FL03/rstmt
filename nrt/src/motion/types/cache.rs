/*
    Appellation: cache <module>
    Contrib: @FL03
*/
use super::Path;
use hashbrown::HashMap;

// Add to Transformer
#[derive(Clone, Debug, PartialEq)]
pub struct PathCache {
    /// Cache of computed paths from one pitch set to another
    paths: HashMap<([usize; 3], usize), Vec<Path>>,
    /// Maximum number of entries to store
    capacity: usize,
    /// Usage counts to implement LRU eviction
    usage: HashMap<([usize; 3], usize), usize>,
}

impl PathCache {
    pub fn new(capacity: usize) -> Self {
        PathCache {
            paths: HashMap::new(),
            capacity,
            usage: HashMap::new(),
        }
    }

    pub fn get(&mut self, from_triad: &[usize; 3], to_pitch: usize) -> Option<&Vec<Path>> {
        let key = (*from_triad, to_pitch);

        // Update usage count
        if let Some(count) = self.usage.get_mut(&key) {
            *count += 1;
        }

        self.paths.get(&key)
    }

    pub fn insert(&mut self, from_triad: [usize; 3], to_pitch: usize, paths: Vec<Path>) {
        let key = (from_triad, to_pitch);

        // If we're at capacity, evict the least used entry
        if self.paths.len() >= self.capacity && !self.paths.contains_key(&key) {
            if let Some((lru_key, _)) = self.usage.iter().min_by_key(|(_, count)| **count) {
                let lru_key = *lru_key;
                self.paths.remove(&lru_key);
                self.usage.remove(&lru_key);
            }
        }

        self.paths.insert(key, paths);
        self.usage.insert(key, 1);
    }
}
