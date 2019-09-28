// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
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
    fn find(&self, k: &Key) -> Option<usize> {
        self.pairs.iter().position(|(x,_)| *x == *k)
    }

    pub fn set(&mut self, k: Key, v: Value) {
        match self.find(&k) {
            Some(i) => self.pairs[i].1 = v,
            None => self.pairs.push((k,v)),
        }
    }
    pub fn get(&self, k: &Key) -> Option<&Value> {
        match self.find(k) {
            Some(i) => Some(&self.pairs[i].1),
            None => None
        }
    }
    pub fn delete(&mut self, k: &Key) {
        match self.find(&k) {
            Some(i) => {
                self.pairs.remove(i);
            },
            None => {}
        }
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    pub fn iter(&self) -> std::slice::Iter<(Key, Value)> {
        self.pairs.iter()
    }
    pub fn entry(&mut self, k: Key) -> PlainMapEntry<Key, Value> {
        PlainMapEntry {
            parent: self,
            k: k
        }
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