// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let m = crate::PlainMap::new();
//     }
// }

#[derive(Clone, Debug)]
pub struct PlainMap<Key, Value> {
    pairs: Vec<(Key, Value)>,

}
impl<Key: PartialEq, Value> PlainMap<Key, Value> {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),

        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pairs: Vec::with_capacity(capacity),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.pairs.reserve(additional)
    }
    pub fn shrink_to_fit(&mut self) {
        self.pairs.shrink_to_fit();
    }


    fn find(&self, k: &Key) -> Option<usize> {
        self.pairs.iter().position(|(x,_)| *x == *k)
    }

    pub fn set(&mut self, k: Key, v: Value) {
        match self.find(&k) {
            Some(i) => self.pairs[i].1 = v,
            None => self.pairs.push((k,v)),
        }
    }
    pub fn insert(&mut self, k: Key, v: Value) -> Option<Value> {
        match self.find(&k) {
            Some(i) => {
                let ov = self.pairs.remove(i).1;
                self.pairs.push((k,v));
                Some(ov)
            },
            None => {
                self.pairs.push((k,v));
                None
            }
        }
    }

    pub fn get(&self, k: &Key) -> Option<&Value> {
        match self.find(k) {
            Some(i) => Some(&self.pairs[i].1),
            None => None
        }
    }
    pub fn get_mut(&mut self, k: &Key) -> Option<&mut Value> {
        match self.find(k) {
            Some(i) => Some(&mut self.pairs[i].1),
            None => None
        }
    }
    pub fn get_key_value(&self, k: &Key) -> Option<&(Key, Value)> {
        match self.find(k) {
            Some(i) => Some(&self.pairs[i]),
            None => None
        }
    }
    pub fn get_key_value_mut(&mut self, k: &Key) -> Option<&mut (Key, Value)> {
        match self.find(k) {
            Some(i) => Some(&mut self.pairs[i]),
            None => None
        }
    }

    pub fn contains_key(&self, k: &Key) -> bool {
        self.find(k).is_some()
    }

    pub fn remove(&mut self, k: &Key) -> Option<Value> {
        match self.find(k) {
            Some(i) => Some(self.pairs.remove(i).1),
            None => None,
        }
    }
    pub fn remove_entry(&mut self, k: &Key) -> Option<(Key, Value)> {
        match self.find(k) {
            Some(i) => Some(self.pairs.remove(i)),
            None => None,
        }
    }

    pub fn entry(&mut self, k: Key) -> PlainMapEntry<Key, Value> {
        PlainMapEntry {
            parent: self,
            k: k
        }
    }

    pub fn capacity(&self) -> usize {
        self.pairs.capacity()
    }
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn clear(&mut self) {
        self.pairs.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<(Key, Value)> {
        self.pairs.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Key, Value)> {
        self.pairs.iter_mut()
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.pairs.iter().map(|(k,_v)| k)
    }
    pub fn keys_mut(&mut self) -> impl Iterator<Item = &mut Key> {
        self.pairs.iter_mut().map(|(k,_v)| k)
    }
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.pairs.iter().map(|(_k,v)| v)
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value> {
        self.pairs.iter_mut().map(|(_k,v)| v)
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, (Key, Value)> {
        self.pairs.drain(..)
    }
}

pub struct PlainMapEntry<'a, Key, Value> {
    parent: &'a mut PlainMap<Key, Value>,
    k: Key,
}
impl<'a, Key: PartialEq, Value> PlainMapEntry<'a, Key, Value> {
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self.parent.find(&self.k) {
            Some(i) => &mut self.parent.pairs[i].1,
            None => {
                self.parent.pairs.push((self.k, default));
                &mut self.parent.pairs.last_mut().unwrap().1
            }
        }
    }
    pub fn or_insert_with<F: FnOnce() -> Value>(self, default: F) -> &'a mut Value {
        match self.parent.find(&self.k) {
            Some(i) => &mut self.parent.pairs[i].1,
            None => {
                self.parent.pairs.push((self.k, default()));
                &mut self.parent.pairs.last_mut().unwrap().1
            }
        }

    }
    pub fn and_modify<F: FnOnce(&mut Value)>(self, f: F) -> Self {
        match self.parent.find(&self.k) {
            Some(i) => f(&mut self.parent.pairs[i].1),
            None => {}
        }
        self
    }
    pub fn key(&self) -> &Key {
        &self.k
    }
}
impl<'a, Key: PartialEq, Value: Default> PlainMapEntry<'a, Key, Value> {
    pub fn or_default(self) -> &'a mut Value {
        self.or_insert_with(Value::default)
    }
}