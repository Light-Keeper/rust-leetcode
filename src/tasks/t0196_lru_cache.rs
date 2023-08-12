// https://leetcode.com/problems/lru-cache/

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct BaseCache<K, V> {
    cache: RefCell<HashMap<K, (V, usize)>>,
    queue: RefCell<VecDeque<K>>,
    capacity: usize
}

impl<K, V> BaseCache<K, V>
    where K: Eq + Copy + Hash,
    V: Copy

{
    fn new(capacity: usize) -> Self {
        BaseCache {
            cache: Default::default(),
            queue: Default::default(),
            capacity
        }
    }

    fn get(&self, key: K) -> Option<V> {
        let mut cache = self.cache.borrow_mut();
        let val = cache.get_mut(&key);

        match val {
            Some((v, len)) => {
                *len += 1;
                self.queue.borrow_mut().push_back(key);
                Some(*v)
            }

            None => {
                None
            }
        }
    }

    fn put(&mut self, key: K, value: V) {
        match self.cache.get_mut().get_mut(&key) {
            Some((v, x)) => {
                *v = value;
                *x += 1;
            }
            None => {
                self.cache.get_mut().insert(key, (value, 1));
            }
        };

        self.queue.get_mut().push_back(key);

        while self.cache.borrow().len() > self.capacity {
            let k = self.queue.borrow_mut().pop_front().unwrap();
            let (_, cnt) = self.cache.get_mut().get_mut(&k).unwrap();
             *cnt -= 1;
            if *cnt == 0 {
                self.cache.get_mut().remove(&k);
            }
        }
    }
}



struct LRUCache {
    cache: BaseCache<i32, i32>
}


impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: BaseCache::new(capacity as usize)
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.cache.get(key).unwrap_or(-1i32)
    }

    fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}