use alloc::vec::Vec;

pub struct SimpleMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> SimpleMap<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    pub fn new() -> Self {
        SimpleMap { data: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(i) = self.data.iter().position(|(k, _)| k == &key) {
            self.data[i].1 = value;
            return;
        }
        self.data.push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in &self.data {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.data.iter().position(|(k, _)| k == key);
        if let Some(i) = index {
            Some(self.data.remove(i).1)
        } else {
            None
        }
    }
}
