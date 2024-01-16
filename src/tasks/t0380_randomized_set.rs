use rand::Rng;
use std::collections::HashMap;

// https://leetcode.com/problems/insert-delete-getrandom-o1
// t0380_randomized_set

#[derive(Default)]
struct RandomizedSet {
    values: Vec<i32>,
    indexes: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        use std::collections::hash_map::Entry::*;

        match self.indexes.entry(val) {
            Occupied(_) => false,
            Vacant(v) => {
                v.insert(self.values.len());
                self.values.push(val);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        use std::collections::hash_map::Entry::*;

        let last_val: i32;
        let deleted_index: usize;

        match self.indexes.entry(val) {
            Occupied(o) => {
                last_val = self.values.pop().unwrap();
                deleted_index = *o.get();
                o.remove_entry();
            }

            Vacant(_) => {
                return false;
            }
        }

        if deleted_index == self.values.len() {
            return true;
        }

        self.values[deleted_index] = last_val;
        self.indexes
            .entry(last_val)
            .and_modify(|e| *e = deleted_index);

        true
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.values.len());
        self.values[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut s = RandomizedSet::new();

        assert!(s.insert(1) == true);
        assert!(s.insert(1) == false);
        assert_eq!(s.get_random(), 1);
        assert!(s.remove(1) == true);
        assert!(s.remove(1) == false);
    }
}
