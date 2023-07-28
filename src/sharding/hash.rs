use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<H>(key: &H) -> u64
where
    H: Hash + ?Sized,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    hasher.finish()
}
