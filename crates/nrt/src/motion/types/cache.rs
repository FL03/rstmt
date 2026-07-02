/*
    Appellation: cache <module>
    Contrib: @FL03
*/
use super::Path;
use core::hash::Hash;
use hashbrown::HashMap;

type PathMap<T> = HashMap<([T; 3], T), Vec<Path<T>>>;
type UsageMap<T> = HashMap<([T; 3], T), usize>;

// Add to Transformer
#[derive(Clone, Debug, PartialEq)]
pub struct PathCache<T = isize>
where
    T: Hash + Eq,
{
    /// Cache of computed paths from one pitch set to another
    pub(crate) paths: PathMap<T>,
    /// Maximum number of entries to store
    pub(crate) capacity: usize,
    /// Usage counts to implement LRU eviction
    pub(crate) usage: UsageMap<T>,
}

impl<T> PathCache<T>
where
    T: Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        PathCache {
            paths: HashMap::new(),
            capacity,
            usage: HashMap::new(),
        }
    }
    /// returns a reference to the paths
    pub const fn paths(&self) -> &PathMap<T> {
        &self.paths
    }
    /// returns a mutable reference to the paths
    pub const fn paths_mut(&mut self) -> &mut PathMap<T> {
        &mut self.paths
    }
    /// returns a copy of the total capacity of the cache
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
    /// returns a reference to the usage map
    pub const fn usage(&self) -> &UsageMap<T> {
        &self.usage
    }
    /// returns a mutable reference to the usage map
    pub const fn usage_mut(&mut self) -> &mut UsageMap<T> {
        &mut self.usage
    }
    /// retrieves paths from the cache, updating usage count if found
    pub fn get(&mut self, from_triad: &[T; 3], to_pitch: T) -> Option<&Vec<Path<T>>>
    where
        T: Copy,
    {
        let key = (*from_triad, to_pitch);

        // Update usage count
        if let Some(count) = self.usage_mut().get_mut(&key) {
            *count += 1;
        }

        self.paths().get(&key)
    }
    /// inserts paths into the cache, evicting least recently used if at capacity
    pub fn insert<I>(&mut self, from_triad: [T; 3], to_pitch: T, paths: I)
    where
        T: Copy,
        I: IntoIterator<Item = Path<T>>,
    {
        let key = (from_triad, to_pitch);

        // If we're at capacity, evict the least used entry
        if self.paths().len() >= self.capacity()
            && !self.paths().contains_key(&key)
            && let Some((lru_key, _)) = self.usage().iter().min_by_key(|(_, count)| **count)
        {
            let lru_key = *lru_key;
            self.paths_mut().remove(&lru_key);
            self.usage_mut().remove(&lru_key);
        }

        self.paths_mut().insert(key, Vec::from_iter(paths));
        self.usage_mut().insert(key, 1);
    }
}
