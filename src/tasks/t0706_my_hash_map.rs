
enum TopLevelEntity {
    Inline(u8, [(i32, i32); 10]),
    OnHeap(Box<[i32; 1024]>)
}

impl Default for TopLevelEntity {
    fn default() -> Self {
        TopLevelEntity::Inline(0, [(0, 0); 10])
    }
}

struct MyHashMap {
    entries: [TopLevelEntity; 1024]
}

fn hash(key: i32) -> (usize, usize) {
    let bytes = key.to_le_bytes();
    let a = ((bytes[0] as usize) << 2) | ((bytes[2] as usize) >> 2);
    let b = ((bytes[1] as usize) << 2) | ((bytes[2] as usize) & 3);
    (a, b)
}

impl MyHashMap {
    fn new() -> Self {
        Self { 
            entries: [1; 1024].map(|_| Default::default())
         }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let (a, b) = hash(key);
        let mut boxed: Option<Box<[i32; 1024]>> = None;

        match &mut self.entries[a] {
            TopLevelEntity::Inline(len, array) => { 
                let index = array.iter()
                    .enumerate()
                    .take(*len as usize)
                    .find(|(_index, &(k, _v))| k == key)
                    .map(|(index, &(_k, _v))| index);
                
                match index {
                    Some(i) => { array[i] = (key, value) },
                    None =>  { 
                        if *len as usize == array.len() {
                            let mut b = Box::new([-1; 1024]);
                            array.iter().for_each(|(k, v)| b[hash(*k).1] = *v);
                            b[hash(key).1] = value;
                            boxed = Some(b);
                        } else {
                            array[*len as usize] = (key, value);
                            *len += 1;
                        }
                    }
                }
            }
            
            TopLevelEntity::OnHeap(heap) => {
                heap[b] = value
            }
        }

        if let Some(b) = boxed {
            self.entries[a] = TopLevelEntity::OnHeap(b);
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        let (a, b) = hash(key);
        match &self.entries[a] {
            TopLevelEntity::Inline(len, array) => { 
                array
                    .iter()
                    .take(*len as usize)
                    .find(|v| v.0 == key)
                    .map(|v| v.1)
                    .unwrap_or(-1)
            }
            TopLevelEntity::OnHeap(vals) => { 
                vals[b]
            }
        }
    }
    
    fn remove(&mut self, key: i32) {
        let (a, b) = hash(key);
        match &mut self.entries[a] {
            TopLevelEntity::Inline(len, array) => { 
                let index = array.iter()
                    .enumerate()
                    .take(*len as usize)
                    .find(|(_index, &(k, _v))| k == key)
                    .map(|(index, &(_k, _v))| index);

                    if let Some(i) = index {
                    *len -= 1;
                    array[i] = array[*len as usize]
                }
            }
            TopLevelEntity::OnHeap(vals) => { 
                vals[b] = -1
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut my_hash_map = super::MyHashMap::new();
        my_hash_map.put(1, 1); // The map is now [[1,1]]
        my_hash_map.put(2, 2); // The map is now [[1,1], [2,2]]
        my_hash_map.get(1);    // return 1, The map is now [[1,1], [2,2]]
        my_hash_map.get(3);    // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        my_hash_map.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        my_hash_map.get(2);    // return 1, The map is now [[1,1], [2,1]]
        my_hash_map.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        my_hash_map.get(2);    // return -1 (i.e., not found), The map is now [[1,1]]
        
    }
}