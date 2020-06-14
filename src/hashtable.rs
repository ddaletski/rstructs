use std::collections::LinkedList;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use colored::Colorize;

#[derive(Debug)]
struct BucketNode<K, V> {
    key: K,
    value: V
}

struct Bucket<K, V> {
    list: LinkedList<BucketNode<K, V>>
}

impl<K, V> Default for Bucket<K, V> {
    fn default() -> Bucket<K, V> {
        Bucket{ list: LinkedList::new() }
    }
}

const DEFAULT_CAPACITY: usize = 16;

pub struct HashTable<K, V> 
{
    table: Vec<Bucket<K, V>>,
    count: usize
}

impl<K, V> HashTable<K, V> 
where K: Eq + std::hash::Hash + std::fmt::Debug,
V: std::fmt::Debug
{
    pub fn new() -> HashTable<K, V> {
        let mut container: Vec<Bucket<K, V>> = Vec::new();
        container.resize_with(DEFAULT_CAPACITY, Default::default);

        HashTable{ table: container, count: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = HashTable::<K, V>::hash_for(&key);

        let pos_in_table = hash as usize % self.nbuckets();
        let bucket = &mut self.table[pos_in_table];

        if let Some(node) = bucket.list.iter_mut().find(|node| { node.key == key }) {
            *node = BucketNode {key, value};
        } else {
            bucket.list.push_front(BucketNode {key, value});
            self.count += 1
        }
    }


    pub fn len(&self) -> usize {
        self.count
    }

    pub fn nbuckets(&self) -> usize {
        self.table.len()
    }

    fn hash_for(key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::HashTable;

    #[test]
    fn default_empty() {
        let ht = HashTable::<i32, i32>::new();
        assert_eq!(ht.count, 0);
        assert_eq!(ht.nbuckets(), super::DEFAULT_CAPACITY);
    }

    #[test]
    fn insert_changes_len() {
        let mut ht = HashTable::<i32, i32>::new();
        let n = 128;
        for i in 0..n {
            ht.insert(i, i*2+1);
            assert_eq!(ht.len(), (i + 1) as usize);
        }
    }

    #[test]
    fn insert_replaces_if_found() {
        let unique_keys = 30;

        let mut ht = HashTable::<i32, i32>::new();
        let n = 128;
        for i in 0..n {
            ht.insert(i % unique_keys, i*2+1);
            assert_eq!(ht.len(), std::cmp::min(i + 1, unique_keys) as usize);
        }
    }
}